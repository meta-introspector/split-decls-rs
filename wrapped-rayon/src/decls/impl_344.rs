macro_rules! deps {
    () => {
        CollectResult!();
    };
}

macro_rules! impl_344 {
    () => {
        deps!();
        unsafe impl < 'c , T > Send for CollectResult < 'c , T > where T : Send { }
    };
}

impl_344!();