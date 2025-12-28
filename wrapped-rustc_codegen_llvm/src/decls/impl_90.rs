macro_rules! deps {
    () => {
        ThinBuffer!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        unsafe impl Sync for ThinBuffer { }
    };
}

impl_90!()