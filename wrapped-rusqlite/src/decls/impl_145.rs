macro_rules! deps {
    () => {
        InnerConnection!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        unsafe impl Send for InnerConnection { }
    };
}

impl_145!()