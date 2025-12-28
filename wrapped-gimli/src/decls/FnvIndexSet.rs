macro_rules! FnvIndexSet {
    () => {
        type FnvIndexSet < T > = indexmap :: IndexSet < T , fnv :: FnvBuildHasher > ;
    };
}

FnvIndexSet!()