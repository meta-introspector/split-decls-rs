macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        unsafe impl Send for Bytes { }
    };
}

impl_68!();