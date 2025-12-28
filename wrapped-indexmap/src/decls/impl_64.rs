macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < K , V , S > Default for IndexMap < K , V , S > where S : Default , { # [doc = " Return an empty [`IndexMap`]"] fn default () -> Self { Self :: with_capacity_and_hasher (0 , S :: default ()) } }
    };
}

impl_64!()