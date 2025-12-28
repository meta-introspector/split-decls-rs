macro_rules! deps {
    () => {
        Own!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        unsafe impl < T > Sync for Own < T > where T : ? Sized { }
    };
}

impl_120!()