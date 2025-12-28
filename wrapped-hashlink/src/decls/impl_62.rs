macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        unsafe impl < K , V > Sync for Iter < '_ , K , V > where K : Sync , V : Sync , { }
    };
}

impl_62!();