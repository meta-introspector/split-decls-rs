// Generated macro for myers_is_even (function)
macro_rules! Depcrate_testsmyers_is_even {
() => {
// Module: crate::tests
// Provides: {"myers_is_even"}
// Dependencies: {}
# [test] fn myers_is_even () { let before = "a\nb\nx\nx\ny\n" ; let after = "b\na\nx\ny\nx\n" ; cov_mark :: check ! (EVEN_SPLIT) ; cov_mark :: check_count ! (SPLIT_SEARCH_ITER , 15) ; let input = InternedInput :: new (before , after) ; let diff = Diff :: compute (Algorithm :: Myers , & input) ; expect ! [[r#"
        @@ -1,5 +1,5 @@
        -a
         b
        -x
        +a
         x
         y
        +x
    "#]] . assert_eq (& diff . unified_diff (& BasicLineDiffPrinter (& input . interner) , UnifiedDiffConfig :: default () , & input ,) . to_string () ,) ; }
};
}
