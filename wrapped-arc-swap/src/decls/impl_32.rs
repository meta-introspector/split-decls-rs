macro_rules! deps {
    () => {
        RefCnt!();
        AsRaw!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < 'a , T : RefCnt > AsRaw < T :: Base > for & 'a T { fn as_raw (& self) -> * mut T :: Base { T :: as_ptr (self) } }
    };
}

impl_32!()