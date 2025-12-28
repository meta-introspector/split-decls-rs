macro_rules! deps {
    () => {
        Version!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        unsafe impl Send for Version { }
    };
}

impl_27!()