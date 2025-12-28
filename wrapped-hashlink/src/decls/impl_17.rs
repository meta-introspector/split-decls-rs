macro_rules! deps {
    () => {
        LinkedHashMap!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        unsafe impl < K : Sync , V : Sync , S : Sync > Sync for LinkedHashMap < K , V , S > { }
    };
}

impl_17!()