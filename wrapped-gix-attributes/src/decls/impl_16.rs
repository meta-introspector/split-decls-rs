macro_rules! deps {
    () => {
        ValueRef!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < 'a > From < & 'a str > for ValueRef < 'a > { fn from (v : & 'a str) -> Self { ValueRef (v . as_bytes ()) } }
    };
}

impl_16!();