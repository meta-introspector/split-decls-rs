macro_rules! deps {
    () => {
        TestDB!();
    };
}

macro_rules! impl_934 {
    () => {
        deps!();
        impl panic :: RefUnwindSafe for TestDB { }
    };
}

impl_934!();