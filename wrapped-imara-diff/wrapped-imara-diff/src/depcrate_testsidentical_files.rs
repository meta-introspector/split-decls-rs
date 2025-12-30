// Generated macro for identical_files (function)
macro_rules! Depcrate_testsidentical_files {
() => {
// Module: crate::tests
// Provides: {"identical_files"}
// Dependencies: {}
# [test] fn identical_files () { let file = r#"fn foo() -> Bar{
    let mut foo = 2.0;
    foo *= 100 / 2;
}"# ; for algorithm in Algorithm :: ALL { println ! ("{algorithm:?}") ; let input = InternedInput :: new (file , file) ; let mut diff = Diff :: compute (algorithm , & input) ; diff . postprocess_lines (& input) ; assert_eq ! (diff . unified_diff (& BasicLineDiffPrinter (& input . interner) , UnifiedDiffConfig :: default () , & input ,) . to_string () , "") ; } }
};
}
