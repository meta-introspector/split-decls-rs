macro_rules! deps {
    () => {
        FxSeededState!();
    };
}

macro_rules! FxHashMapSeed {
    () => {
        deps!();
        # [doc = " Type alias for a hashmap using the `fx` hash algorithm with [`FxSeededState`]."] # [cfg (feature = "std")] pub type FxHashMapSeed < K , V > = std :: collections :: HashMap < K , V , FxSeededState > ;
    };
}

FxHashMapSeed!();