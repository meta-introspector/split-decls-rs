macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        unsafe impl < K , V > Sync for IterMut < '_ , K , V > where K : Sync , V : Sync , { }
    };
}

impl_63!();