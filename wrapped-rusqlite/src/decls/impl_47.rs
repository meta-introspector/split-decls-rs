macro_rules! deps {
    () => {
        Connection!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        unsafe impl Send for Connection { }
    };
}

impl_47!()