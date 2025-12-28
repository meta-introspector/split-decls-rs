macro_rules! deps {
    () => {
        Guard!();
        RefCnt!();
        AsRaw!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < 'a , T : RefCnt > AsRaw < T :: Base > for & 'a Guard < T > { fn as_raw (& self) -> * mut T :: Base { T :: as_ptr (self) } }
    };
}

impl_34!();