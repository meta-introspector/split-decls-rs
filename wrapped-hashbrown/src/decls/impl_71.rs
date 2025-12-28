macro_rules! deps {
    () => {
        RawIterRange!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        unsafe impl < T > Sync for RawIterRange < T > { }
    };
}

impl_71!();