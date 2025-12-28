macro_rules! FnvIndexMap {
    () => {
        type FnvIndexMap < T , V > = indexmap :: IndexMap < T , V , fnv :: FnvBuildHasher > ;
    };
}

FnvIndexMap!()