macro_rules! deps {
    () => {
        DeclarativeMacro!();
    };
}

macro_rules! macro_rules_fixtures {
    () => {
        deps!();
        fn macro_rules_fixtures () -> FxHashMap < String , DeclarativeMacro > { macro_rules_fixtures_tt () . into_iter () . sorted_by_key (| (id , _) | id . clone ()) . map (| (id , tt) | (id , DeclarativeMacro :: parse_macro_rules (& tt , | _ | span :: Edition :: CURRENT))) . collect () }
    };
}

macro_rules_fixtures!();