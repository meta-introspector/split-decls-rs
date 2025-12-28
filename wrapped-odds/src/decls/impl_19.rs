macro_rules! deps {
    () => {
        IndexRange!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < T > IndexRange < T > for RangeFull { }
    };
}

impl_19!()