macro_rules! deps {
    () => {
        JobRef!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        unsafe impl Sync for JobRef { }
    };
}

impl_35!();