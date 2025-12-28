macro_rules! deps {
    () => {
        FxBuildHasher!();
    };
}

macro_rules! FxHashSet {
    () => {
        deps!();
        # [doc = " Type alias for a hash set that uses the Fx hashing algorithm."] # [cfg (feature = "std")] pub type FxHashSet < V > = HashSet < V , FxBuildHasher > ;
    };
}

FxHashSet!()