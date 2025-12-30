// Generated macro for benchmark_main (macro)
macro_rules! Depcratebenchmark_main {
() => {
// Module: crate
// Provides: {"benchmark_main"}
// Dependencies: {}
# [doc = " Stand-in for `bencher::benchmark_main!` which performs benchmarks using Criterion.rs instead."] # [macro_export] macro_rules ! benchmark_main { ($ ($ group_name : path) ,+) => { fn main () { $ ($ group_name () ;) + $ crate :: Criterion :: default () . configure_from_args () . final_summary () ; } } ; ($ ($ group_name : path ,) +) => { benchmark_main ! ($ ($ group_name) ,+) ; } ; }
};
}
