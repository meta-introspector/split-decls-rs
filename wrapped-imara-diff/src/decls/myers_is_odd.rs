macro_rules! deps {
    () => {
        InternedInput!();
        UnifiedDiffConfig!();
        BasicLineDiffPrinter!();
        Diff!();
        Algorithm!();
        Myers!();
    };
}

macro_rules! myers_is_odd {
    () => {
        deps!();
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

myers_is_odd!();