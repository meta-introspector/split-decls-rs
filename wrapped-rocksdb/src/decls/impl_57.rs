macro_rules! deps {
    () => {
        ColumnFamily!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        unsafe impl Send for ColumnFamily { }
    };
}

impl_57!();