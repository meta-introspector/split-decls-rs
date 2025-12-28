macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        unsafe impl < K , V > Sync for IntoIter < K , V > where K : Sync , V : Sync , { }
    };
}

impl_64!()