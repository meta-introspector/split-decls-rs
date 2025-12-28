macro_rules! deps {
    () => {
        BoundColumnFamily!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        unsafe impl Send for BoundColumnFamily < '_ > { }
    };
}

impl_61!()