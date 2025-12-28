macro_rules! deps {
    () => {
        Unparker!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        unsafe impl Sync for Unparker { }
    };
}

impl_101!()