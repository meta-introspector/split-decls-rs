macro_rules! deps {
    () => {
        FxSeededState!();
    };
}

macro_rules! FxHashSetSeed {
    () => {
        deps!();
        # [doc = " Type alias for a hashmap using the `fx` hash algorithm with [`FxSeededState`]."] # [cfg (feature = "std")] pub type FxHashSetSeed < V > = std :: collections :: HashSet < V , FxSeededState > ;
    };
}

FxHashSetSeed!();