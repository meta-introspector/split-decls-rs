macro_rules! FxIndexMap {
    () => {
        pub type FxIndexMap < K , V > = rustc_type_ir :: data_structures :: IndexMap < K , V > ;
    };
}

FxIndexMap!()