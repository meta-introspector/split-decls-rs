macro_rules! deps {
    () => {
        ColumnFamily!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        unsafe impl Sync for ColumnFamily { }
    };
}

impl_58!()