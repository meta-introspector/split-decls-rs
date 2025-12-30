// Generated macro for patch (function)
macro_rules! Depcrate_integrated_benchmarkspatch {
() => {
// Module: crate::integrated_benchmarks
// Provides: {"patch"}
// Dependencies: {}
fn patch (what : & mut String , from : & str , to : & str) -> usize { let idx = what . find (from) . unwrap () ; * what = what . replacen (from , to , 1) ; idx }
};
}
