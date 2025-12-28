macro_rules! deps {
    () => {
        AsRaw!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < T > AsRaw < T > for * mut T { fn as_raw (& self) -> * mut T { * self } }
    };
}

impl_38!()