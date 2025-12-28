macro_rules! deps {
    () => {
        TreeUpdateBuilder!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        unsafe impl Send for TreeUpdateBuilder { }
    };
}

impl_102!();