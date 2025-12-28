macro_rules! deps {
    () => {
        Version!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        unsafe impl Sync for Version { }
    };
}

impl_28!()