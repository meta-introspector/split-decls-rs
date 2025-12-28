macro_rules! FxIndexMap {
    () => {
        type FxIndexMap < K , V > = indexmap :: IndexMap < K , V , rustc_hash :: FxBuildHasher > ;
    };
}

FxIndexMap!();