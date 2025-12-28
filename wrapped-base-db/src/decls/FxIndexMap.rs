macro_rules! FxIndexMap {
    () => {
        pub type FxIndexMap < K , V > = indexmap :: IndexMap < K , V , std :: hash :: BuildHasherDefault < rustc_hash :: FxHasher > > ;
    };
}

FxIndexMap!()