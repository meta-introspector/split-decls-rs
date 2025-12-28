macro_rules! deps {
    () => {
        ThinBuffer!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        unsafe impl Send for ThinBuffer { }
    };
}

impl_89!()