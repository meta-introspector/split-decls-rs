macro_rules! deps {
    () => {
        RawOccupiedEntryMut!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        unsafe impl < K , V , S > Sync for RawOccupiedEntryMut < '_ , K , V , S > where K : Sync , V : Sync , S : Sync , { }
    };
}

impl_48!();