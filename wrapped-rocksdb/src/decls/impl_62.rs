macro_rules! deps {
    () => {
        BoundColumnFamily!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        unsafe impl Sync for BoundColumnFamily < '_ > { }
    };
}

impl_62!();