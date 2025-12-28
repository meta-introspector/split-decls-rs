macro_rules! deps {
    () => {
        UnboundColumnFamily!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        unsafe impl Sync for UnboundColumnFamily { }
    };
}

impl_60!()