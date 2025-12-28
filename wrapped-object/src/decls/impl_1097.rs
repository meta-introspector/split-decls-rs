macro_rules! deps {
    () => {
        Table!();
    };
}

macro_rules! impl_1097 {
    () => {
        deps!();
        impl < T > Table < T > { pub (super) fn new () -> Self { Table (Vec :: new ()) } }
    };
}

impl_1097!()