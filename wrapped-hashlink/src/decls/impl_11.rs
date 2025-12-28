macro_rules! deps {
    () => {
        LinkedHashMap!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < K : Hash + Eq , V : Eq , S : BuildHasher > Eq for LinkedHashMap < K , V , S > { }
    };
}

impl_11!();