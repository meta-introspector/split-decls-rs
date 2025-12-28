macro_rules! add_dep {
    () => {
        fn add_dep (graph : & mut CrateGraphBuilder , from : CrateBuilderId , name : CrateName , to : CrateBuilderId ,) { add_dep_inner (graph , from , DependencyBuilder :: new (name , to)) }
    };
}

add_dep!()