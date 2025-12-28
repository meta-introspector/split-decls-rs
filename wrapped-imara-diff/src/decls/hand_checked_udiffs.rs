macro_rules! deps {
    () => {
        BasicLineDiffPrinter!();
        Diff!();
        UnifiedDiffConfig!();
        Algorithm!();
        InternedInput!();
    };
}

macro_rules! hand_checked_udiffs {
    () => {
        deps!();
        # [test] # [cfg (not (miri))] fn hand_checked_udiffs () { for algorithm in Algorithm :: ALL { println ! ("{algorithm:?}") ; let test_dir = project_root () . join ("tests") ; let file = "helix_syntax.rs" ; let path_before = test_dir . join (format ! ("{file}.before")) ; let path_after = test_dir . join (format ! ("{file}.after")) ; let path_diff = test_dir . join (format ! ("{file}.{algorithm:?}.diff")) ; let before = read_to_string (path_before) . unwrap () ; let after = read_to_string (path_after) . unwrap () ; let input = InternedInput :: new (& * before , & * after) ; let mut diff = Diff :: compute (algorithm , & input) ; diff . postprocess_lines (& input) ; expect_file ! [path_diff] . assert_eq (& diff . unified_diff (& BasicLineDiffPrinter (& input . interner) , UnifiedDiffConfig :: default () , & input ,) . to_string () ,) ; } }
    };
}

hand_checked_udiffs!()