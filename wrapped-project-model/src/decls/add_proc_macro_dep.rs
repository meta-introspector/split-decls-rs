macro_rules! add_proc_macro_dep {
    () => {
        fn add_proc_macro_dep (crate_graph : & mut CrateGraphBuilder , from : CrateBuilderId , to : CrateBuilderId , prelude : bool ,) { add_dep_with_prelude (crate_graph , from , CrateName :: new ("proc_macro") . unwrap () , to , prelude , true ,) ; }
    };
}

add_proc_macro_dep!()