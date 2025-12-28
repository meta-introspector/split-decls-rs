macro_rules! deps {
    () => {
        Collector!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        unsafe impl Sync for Collector { }
    };
}

impl_68!()