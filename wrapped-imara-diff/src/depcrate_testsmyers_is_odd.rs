// Generated macro for myers_is_odd (function)
macro_rules! Depcrate_testsmyers_is_odd {
() => {
// Module: crate::tests
// Provides: {"myers_is_odd"}
// Dependencies: {}
# [test] fn myers_is_odd () { let before = "a\nb\nx\ny\nx\n" ; let after = "b\na\nx\ny\n" ; cov_mark :: check ! (ODD_SPLIT) ; cov_mark :: check_count ! (SPLIT_SEARCH_ITER , 9) ; let input = InternedInput :: new (before , after) ; let diff = Diff :: compute (Algorithm :: Myers , & input) ; expect ! [[r#"
        @@ -1,5 +1,4 @@
        -a
         b
        +a
         x
         y
        -x
    "#]] . assert_eq (& diff . unified_diff (& BasicLineDiffPrinter (& input . interner) , UnifiedDiffConfig :: default () , & input ,) . to_string () ,) ; }
};
}
