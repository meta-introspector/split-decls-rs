macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        unsafe impl < K , V > Send for Drain < '_ , K , V > where K : Send , V : Send , { }
    };
}

impl_61!();