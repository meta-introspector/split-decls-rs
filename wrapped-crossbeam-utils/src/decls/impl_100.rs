macro_rules! deps {
    () => {
        Unparker!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        unsafe impl Send for Unparker { }
    };
}

impl_100!()