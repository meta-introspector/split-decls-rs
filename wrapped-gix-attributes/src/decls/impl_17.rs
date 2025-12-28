macro_rules! deps {
    () => {
        Value!();
        ValueRef!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < 'a > From < ValueRef < 'a > > for Value { fn from (v : ValueRef < 'a >) -> Self { Value (v . 0 . into ()) } }
    };
}

impl_17!();