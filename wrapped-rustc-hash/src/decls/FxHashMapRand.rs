macro_rules! deps {
    () => {
        FxRandomState!();
    };
}

macro_rules! FxHashMapRand {
    () => {
        deps!();
        # [doc = " Type alias for a hashmap using the `fx` hash algorithm with [`FxRandomState`]."] pub type FxHashMapRand < K , V > = HashMap < K , V , FxRandomState > ;
    };
}

FxHashMapRand!()