macro_rules! deps {
    () => {
        RawVacantEntryMut!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        unsafe impl < K , V , S > Send for RawVacantEntryMut < '_ , K , V , S > where K : Send , V : Send , S : Send , { }
    };
}

impl_49!();