macro_rules! deps {
    () => {
        Compress!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        unsafe impl Sync for Compress { }
    };
}

impl_102!()