macro_rules! deps {
    () => {
        IndexRange!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < T > IndexRange < T > for RangeFull { }
    };
}

impl_63!();