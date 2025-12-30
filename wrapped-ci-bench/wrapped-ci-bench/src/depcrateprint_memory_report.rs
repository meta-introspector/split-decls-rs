// Generated macro for print_memory_report (function)
macro_rules! Depcrateprint_memory_report {
() => {
// Module: crate
// Provides: {"print_memory_report"}
// Dependencies: {}
fn print_memory_report (result : & MemoryCompareResult) { println ! ("# Memory measurement results") ; if ! result . missing_in_baseline . is_empty () { println ! ("### ⚠️ Warning: missing benchmarks") ; println ! () ; println ! ("The following benchmark scenarios are present in the candidate but not in the baseline:") ; println ! () ; for scenario in & result . missing_in_baseline { println ! ("* {scenario}") ; } } println ! ("## Memory measurement differences") ; if result . diffs . is_empty () { println ! ("_There are no memory measurement differences_") ; } else { memory_table (& result . diffs , true) ; } }
};
}
