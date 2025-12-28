macro_rules! deps {
    () => {
        TestDB!();
    };
}

macro_rules! impl_590 {
    () => {
        deps!();
        impl panic :: RefUnwindSafe for TestDB { }
    };
}

impl_590!()