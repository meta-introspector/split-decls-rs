macro_rules! add_dep_inner {
    () => {
        fn add_dep_inner (graph : & mut CrateGraphBuilder , from : CrateBuilderId , dep : DependencyBuilder) { if let Err (err) = graph . add_dep (from , dep) { tracing :: warn ! ("{}" , err) } }
    };
}

add_dep_inner!();