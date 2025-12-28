macro_rules! deps {
    () => {
        UserBody!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        unsafe impl Send for UserBody { }
    };
}

impl_249!();