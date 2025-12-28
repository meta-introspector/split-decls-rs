macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        unsafe impl < K , V > Sync for Drain < '_ , K , V > where K : Sync , V : Sync , { }
    };
}

impl_65!();