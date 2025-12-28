macro_rules! deps {
    () => {
        Connection!();
    };
}

macro_rules! impl_702 {
    () => {
        deps!();
        unsafe impl Send for Connection { }
    };
}

impl_702!()