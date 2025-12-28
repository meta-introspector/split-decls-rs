macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        unsafe impl < K , V > Send for IntoIter < K , V > where K : Send , V : Send , { }
    };
}

impl_60!()