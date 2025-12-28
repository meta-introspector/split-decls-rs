macro_rules! deps {
    () => {
        Gen!();
        Arbitrary!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Arbitrary for () { fn arbitrary (_ : & mut Gen) { } }
    };
}

impl_10!();