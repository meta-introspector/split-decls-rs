macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_251 {
    () => {
        deps!();
        unsafe impl < K : Send , V : Send > Send for IterMut < '_ , K , V > { }
    };
}

impl_251!()