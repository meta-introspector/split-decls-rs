macro_rules! deps {
    () => {
        FxRandomState!();
    };
}

macro_rules! FxHashSetRand {
    () => {
        deps!();
        # [doc = " Type alias for a hashmap using the `fx` hash algorithm with [`FxRandomState`]."] pub type FxHashSetRand < V > = HashSet < V , FxRandomState > ;
    };
}

FxHashSetRand!();