macro_rules! deps {
    () => {
        UserDataPointer!();
    };
}

macro_rules! impl_377 {
    () => {
        deps!();
        unsafe impl Send for UserDataPointer { }
    };
}

impl_377!();