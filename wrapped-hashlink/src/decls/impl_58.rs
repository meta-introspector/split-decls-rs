macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        unsafe impl < K , V > Send for Iter < '_ , K , V > where K : Sync , V : Sync , { }
    };
}

impl_58!()