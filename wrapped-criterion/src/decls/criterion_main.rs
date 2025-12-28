macro_rules! deps {
    () => {
        Criterion!();
    };
}

macro_rules! criterion_main {
    () => {
        deps!();
        # [doc = " Macro which expands to a benchmark harness."] # [doc = ""] # [doc = " Currently, using Criterion.rs requires disabling the benchmark harness"] # [doc = " generated automatically by rustc. This can be done like so:"] # [doc = ""] # [doc = " ```toml"] # [doc = " [[bench]]"] # [doc = " name = \"my_bench\""] # [doc = " harness = false"] # [doc = " ```"] # [doc = ""] # [doc = " In this case, `my_bench` must be a rust file inside the 'benches' directory,"] # [doc = " like so:"] # [doc = ""] # [doc = " `benches/my_bench.rs`"] # [doc = ""] # [doc = " Since we've disabled the default benchmark harness, we need to add our own:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use criterion::{criterion_group, criterion_main, Criterion};"] # [doc = " fn bench_method1(c: &mut Criterion) {"] # [doc = " }"] # [doc = ""] # [doc = " fn bench_method2(c: &mut Criterion) {"] # [doc = " }"] # [doc = ""] # [doc = " criterion_group!(benches, bench_method1, bench_method2);"] # [doc = " criterion_main!(benches);"] # [doc = " ```"] # [doc = ""] # [doc = " The `criterion_main` macro expands to a `main` function which runs all of the"] # [doc = " benchmarks in the given groups."] # [doc = ""] # [macro_export] macro_rules ! criterion_main { ($ ($ group : path) ,+ $ (,) *) => { fn main () { $ ($ group () ;) + $ crate :: Criterion :: default () . configure_from_args () . final_summary () ; } } }
    };
}

criterion_main!()