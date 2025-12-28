macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! impl_695 {
    () => {
        deps!();
        unsafe impl Send for Repository { }
    };
}

impl_695!();