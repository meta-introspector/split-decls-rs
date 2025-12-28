macro_rules! deps {
    () => {
        Gen!();
        Arbitrary!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl Arbitrary for RangeFull { fn arbitrary (_ : & mut Gen) -> RangeFull { .. } }
    };
}

impl_63!();