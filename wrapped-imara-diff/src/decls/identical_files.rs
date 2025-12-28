macro_rules! deps {
    () => {
        Diff!();
        Algorithm!();
        InternedInput!();
        UnifiedDiffConfig!();
        BasicLineDiffPrinter!();
    };
}

macro_rules! identical_files {
    () => {
        deps!();
        # [test] fn identical_files () { let file = r#"fn foo() -> Bar{
    let mut foo = 2.0;
    foo *= 100 / 2;
}"# ; for algorithm in Algorithm :: ALL { println ! ("{algorithm:?}") ; let input = InternedInput :: new (file , file) ; let mut diff = Diff :: compute (algorithm , & input) ; diff . postprocess_lines (& input) ; assert_eq ! (diff . unified_diff (& BasicLineDiffPrinter (& input . interner) , UnifiedDiffConfig :: default () , & input ,) . to_string () , "") ; } }
    };
}

identical_files!();