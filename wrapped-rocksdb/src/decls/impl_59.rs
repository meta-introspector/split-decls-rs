macro_rules! deps {
    () => {
        UnboundColumnFamily!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        unsafe impl Send for UnboundColumnFamily { }
    };
}

impl_59!()