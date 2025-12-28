macro_rules! deps {
    () => {
        StatementCache!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        unsafe impl Send for StatementCache { }
    };
}

impl_62!();