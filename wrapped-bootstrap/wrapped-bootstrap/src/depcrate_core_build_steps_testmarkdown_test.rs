// Generated macro for markdown_test (function)
macro_rules! Depcrate_core_build_steps_testmarkdown_test {
() => {
// Module: crate::core::build_steps::test
// Provides: {"markdown_test"}
// Dependencies: {}
fn markdown_test (builder : & Builder < '_ > , compiler : Compiler , markdown : & Path) -> bool { if let Ok (contents) = fs :: read_to_string (markdown) && ! contents . contains ("```") { return true ; } builder . verbose (| | println ! ("doc tests for: {}" , markdown . display ())) ; let mut cmd = builder . rustdoc_cmd (compiler) ; builder . add_rust_test_threads (& mut cmd) ; cmd . arg ("-Z") ; cmd . arg ("unstable-options") ; cmd . arg ("--test") ; cmd . arg (markdown) ; cmd . env ("RUSTC_BOOTSTRAP" , "1") ; let test_args = builder . config . test_args () . join (" ") ; cmd . arg ("--test-args") . arg (test_args) ; cmd = cmd . delay_failure () ; if ! builder . config . verbose_tests { cmd . run_capture (builder) . is_success () } else { cmd . run (builder) } }
};
}
