macro_rules! add_dep_with_prelude {
    () => {
        fn add_dep_with_prelude (graph : & mut CrateGraphBuilder , from : CrateBuilderId , name : CrateName , to : CrateBuilderId , prelude : bool , sysroot : bool ,) { add_dep_inner (graph , from , DependencyBuilder :: with_prelude (name , to , prelude , sysroot)) }
    };
}

add_dep_with_prelude!()