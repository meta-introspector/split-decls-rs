macro_rules! deps {
    () => {
        RefCnt!();
        Guard!();
        AsRaw!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < T : RefCnt > AsRaw < T :: Base > for Guard < T > { fn as_raw (& self) -> * mut T :: Base { T :: as_ptr (self) } }
    };
}

impl_36!();