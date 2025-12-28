macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        unsafe impl Sync for Bytes { }
    };
}

impl_69!();