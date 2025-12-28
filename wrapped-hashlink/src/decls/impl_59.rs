macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        unsafe impl < K , V > Send for IterMut < '_ , K , V > where K : Send , V : Send , { }
    };
}

impl_59!()