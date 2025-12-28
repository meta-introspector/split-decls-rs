macro_rules! deps {
    () => {
        FxBuildHasher!();
    };
}

macro_rules! FxHashMap {
    () => {
        deps!();
        # [doc = " Type alias for a hash map that uses the Fx hashing algorithm."] # [cfg (feature = "std")] pub type FxHashMap < K , V > = HashMap < K , V , FxBuildHasher > ;
    };
}

FxHashMap!()