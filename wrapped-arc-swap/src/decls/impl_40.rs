macro_rules! deps {
    () => {
        AsRaw!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < T > AsRaw < T > for * const T { fn as_raw (& self) -> * mut T { * self as * mut T } }
    };
}

impl_40!()