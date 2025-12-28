macro_rules! check_crate_graph {
    () => {
        fn check_crate_graph (crate_graph : CrateGraphBuilder , expect : ExpectFile) { let mut crate_graph = format ! ("{crate_graph:#?}") ; replace_root (& mut crate_graph , false) ; replace_cargo (& mut crate_graph) ; expect . assert_eq (& crate_graph) ; }
    };
}

check_crate_graph!()