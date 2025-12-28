macro_rules! benchmark_expand_macro_rules {
    () => {
        # [test] fn benchmark_expand_macro_rules () { if skip_slow_tests () { return ; } let rules = macro_rules_fixtures () ; let invocations = invocation_fixtures (& rules) ; let hash : usize = { let _pt = bench ("mbe expand macro rules") ; invocations . into_iter () . map (| (id , tt) | { let res = rules [& id] . expand (& tt , | _ | () , DUMMY , Edition :: CURRENT) ; assert ! (res . err . is_none ()) ; res . value . 0 . 0 . len () }) . sum () } ; assert_eq ! (hash , 450144) ; }
    };
}

benchmark_expand_macro_rules!()