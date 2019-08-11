#[macro_use]
extern crate criterion;

extern crate bls12_381;
use bls12_381::*;

use criterion::{black_box, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    // G1Affine
    {
        let name = "G1Affine";
        let a = G1Affine::generator();
        c.bench_function(&format!("{} check on curve", name), move |b| {
            b.iter(|| black_box(a).is_on_curve())
        });
        c.bench_function(&format!("{} check equality", name), move |b| {
            b.iter(|| black_box(a) == black_box(a))
        });
    }

    // G1Projective
    {
        let name = "G1Projective";
        let a = G1Projective::generator();
        let a_affine = G1Affine::generator();
        c.bench_function(&format!("{} check on curve", name), move |b| {
            b.iter(|| black_box(a).is_on_curve())
        });
        c.bench_function(&format!("{} check equality", name), move |b| {
            b.iter(|| black_box(a) == black_box(a))
        });
        c.bench_function(&format!("{} to affine", name), move |b| {
            b.iter(|| G1Affine::from(black_box(a)))
        });
        c.bench_function(&format!("{} doubling", name), move |b| {
            b.iter(|| black_box(a).double())
        });
        c.bench_function(&format!("{} addition", name), move |b| {
            b.iter(|| black_box(a).add(&a))
        });
        c.bench_function(&format!("{} mixed addition", name), move |b| {
            b.iter(|| black_box(a).add_mixed(&a_affine))
        });
    }

    // G2Affine
    {
        let name = "G2Affine";
        let a = G2Affine::generator();
        c.bench_function(&format!("{} check on curve", name), move |b| {
            b.iter(|| black_box(a).is_on_curve())
        });
        c.bench_function(&format!("{} check equality", name), move |b| {
            b.iter(|| black_box(a) == black_box(a))
        });
    }

    // G2Projective
    {
        let name = "G2Projective";
        let a = G2Projective::generator();
        let a_affine = G2Affine::generator();
        c.bench_function(&format!("{} check on curve", name), move |b| {
            b.iter(|| black_box(a).is_on_curve())
        });
        c.bench_function(&format!("{} check equality", name), move |b| {
            b.iter(|| black_box(a) == black_box(a))
        });
        c.bench_function(&format!("{} to affine", name), move |b| {
            b.iter(|| G2Affine::from(black_box(a)))
        });
        c.bench_function(&format!("{} doubling", name), move |b| {
            b.iter(|| black_box(a).double())
        });
        c.bench_function(&format!("{} addition", name), move |b| {
            b.iter(|| black_box(a).add(&a))
        });
        c.bench_function(&format!("{} mixed addition", name), move |b| {
            b.iter(|| black_box(a).add_mixed(&a_affine))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
