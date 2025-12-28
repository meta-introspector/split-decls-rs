macro_rules! deps {
    () => {
        DeclarativeMacro!();
    };
}

macro_rules! benchmark_parse_macro_rules {
    () => {
        deps!();
        # [test] fn benchmark_parse_macro_rules () { if skip_slow_tests () { return ; } let rules = macro_rules_fixtures_tt () ; let hash : usize = { let _pt = bench ("mbe parse macro rules") ; rules . into_iter () . sorted_by_key (| (id , _) | id . clone ()) . map (| (_ , it) | { DeclarativeMacro :: parse_macro_rules (& it , | _ | span :: Edition :: CURRENT) . rules . len () }) . sum () } ; assert_eq ! (hash , 1144) ; }
    };
}

benchmark_parse_macro_rules!();