macro_rules! deps {
    () => {
        RawVacantEntryMut!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        unsafe impl < K , V , S > Sync for RawVacantEntryMut < '_ , K , V , S > where K : Sync , V : Sync , S : Sync , { }
    };
}

impl_50!();