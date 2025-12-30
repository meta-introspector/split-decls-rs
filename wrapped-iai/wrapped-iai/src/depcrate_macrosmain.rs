// Generated macro for main (macro)
macro_rules! Depcrate_macrosmain {
() => {
// Module: crate::macros
// Provides: {"main"}
// Dependencies: {}
# [doc = " Macro which expands to a benchmark harness."] # [doc = ""] # [doc = " Currently, using Iai requires disabling the benchmark harness"] # [doc = " generated automatically by rustc. This can be done like so:"] # [doc = ""] # [doc = " ```toml"] # [doc = " [[bench]]"] # [doc = " name = \"my_bench\""] # [doc = " harness = false"] # [doc = " ```"] # [doc = ""] # [doc = " In this case, `my_bench` must be a rust file inside the 'benches' directory,"] # [doc = " like so:"] # [doc = ""] # [doc = " `benches/my_bench.rs`"] # [doc = ""] # [doc = " Since we've disabled the default benchmark harness, we need to add our own:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " fn bench_method1() {"] # [doc = " }"] # [doc = ""] # [doc = " fn bench_method2() {"] # [doc = " }"] # [doc = ""] # [doc = " iai::main!(bench_method1, bench_method2);"] # [doc = " ```"] # [doc = ""] # [doc = " The `iai::main` macro expands to a `main` function which runs all of the"] # [doc = " benchmarks in the given groups."] # [doc = ""] # [macro_export] macro_rules ! main { ($ ($ func_name : ident) ,+ $ (,) *) => { mod iai_wrappers { $ (pub fn $ func_name () { let _ = $ crate :: black_box (super ::$ func_name ()) ; }) + } fn main () { let benchmarks : & [& (&'static str , fn ())] = & [$ (& (stringify ! ($ func_name) , iai_wrappers ::$ func_name) ,) +] ; $ crate :: runner (benchmarks) ; } } }
};
}
