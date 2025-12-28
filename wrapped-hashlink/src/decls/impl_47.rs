macro_rules! deps {
    () => {
        RawOccupiedEntryMut!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        unsafe impl < K , V , S > Send for RawOccupiedEntryMut < '_ , K , V , S > where K : Send , V : Send , S : Send , { }
    };
}

impl_47!()