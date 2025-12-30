// Generated macro for show_usage (function)
macro_rules! Depcrate_testshow_usage {
() => {
// Module: crate::test
// Provides: {"show_usage"}
// Dependencies: {}
fn show_usage () { println ! (r#"
`test` command help:

    --release              : Build codegen in release mode
    --sysroot-panic-abort  : Build the sysroot without unwinding support.
    --features [arg]       : Add a new feature [arg]
    --use-system-gcc       : Use system installed libgccjit
    --build-only           : Only build rustc_codegen_gcc then exits
    --nb-parts             : Used to split rustc_tests (for CI needs)
    --current-part         : Used with `--nb-parts`, allows you to specify which parts to test"#) ; ConfigInfo :: show_usage () ; for (option , (doc , _)) in get_runners () { let needed_spaces = 23_usize . saturating_sub (option . len ()) ; let spaces : String = std :: iter :: repeat_n (' ' , needed_spaces) . collect () ; println ! ("    {option}{spaces}: {doc}") ; } println ! ("    --help                 : Show this help") ; }
};
}
