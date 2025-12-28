macro_rules! FxIndexSet {
    () => {
        pub type FxIndexSet < T > = indexmap :: IndexSet < T , rustc_hash :: FxBuildHasher > ;
    };
}

FxIndexSet!()