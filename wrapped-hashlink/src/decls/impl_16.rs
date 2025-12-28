macro_rules! deps {
    () => {
        LinkedHashMap!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        unsafe impl < K : Send , V : Send , S : Send > Send for LinkedHashMap < K , V , S > { }
    };
}

impl_16!();