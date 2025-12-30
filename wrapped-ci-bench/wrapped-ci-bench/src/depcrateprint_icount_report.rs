// Generated macro for print_icount_report (function)
macro_rules! Depcrateprint_icount_report {
() => {
// Module: crate
// Provides: {"print_icount_report"}
// Dependencies: {}
# [doc = " Prints a report of the comparison to stdout, using GitHub-flavored markdown"] fn print_icount_report (result : & CompareResult) { println ! ("# Benchmark results") ; if ! result . missing_in_baseline . is_empty () { println ! ("### ⚠️ Warning: missing benchmarks") ; println ! () ; println ! ("The following benchmark scenarios are present in the candidate but not in the baseline:") ; println ! () ; for scenario in & result . missing_in_baseline { println ! ("* {scenario}") ; } } println ! ("## Instruction count differences") ; if result . diffs . is_empty () { println ! ("_There are no instruction count differences_") ; } else { table (result . diffs . iter () . map (| (diff , _) | diff) , true ,) ; println ! ("<details>") ; println ! ("<summary>Details per scenario</summary>\n") ; for (diff , detailed_diff) in & result . diffs { println ! ("#### {}" , diff . scenario) ; println ! ("```") ; println ! ("{detailed_diff}") ; println ! ("```") ; } println ! ("</details>\n") } }
};
}
