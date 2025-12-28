macro_rules! deps {
    () => {
        Algorithm!();
        Diff!();
        BasicLineDiffPrinter!();
        InternedInput!();
        Histogram!();
        Myers!();
        UnifiedDiffConfig!();
    };
}

macro_rules! postprocess {
    () => {
        deps!();
        # [test] fn postprocess () { let before = r#"
       /*
        * Stay on the safe side. if read_directory() has run once on
        * "dir", some sticky flag may have been left. Clear them all.
        */
       clear_sticky(dir);

       /*
        * exclude patterns are treated like positive ones in
        * create_simplify. Usually exclude patterns should be a
        * subset of positive ones, which has no impacts on
        * foo
        * bar
        * test
        */
        foo
    "# ; let after = r#"
       /*
        * exclude patterns are treated like positive ones in
        * create_simplify. Usually exclude patterns should be a
        * subset of positive ones, which has no impacts on
        * foo
        * bar
        * test
        */
        foo
    "# ; let input = InternedInput :: new (before , after) ; for algorithm in [Algorithm :: Histogram , Algorithm :: Myers] { println ! ("{algorithm:?}") ; let mut diff = Diff :: compute (algorithm , & input) ; diff . postprocess_lines (& input) ; let diff = diff . unified_diff (& BasicLineDiffPrinter (& input . interner) , UnifiedDiffConfig :: default () , & input ,) . to_string () ; println ! ("{diff:?}") ; expect ! [[r#"
            @@ -1,10 +1,4 @@
             
            -       /*
            -        * Stay on the safe side. if read_directory() has run once on
            -        * "dir", some sticky flag may have been left. Clear them all.
            -        */
            -       clear_sticky(dir);
            -
                    /*
                     * exclude patterns are treated like positive ones in
                     * create_simplify. Usually exclude patterns should be a
        "#]] . assert_eq (& diff) ; } }
    };
}

postprocess!();