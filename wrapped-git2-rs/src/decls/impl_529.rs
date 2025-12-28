macro_rules! deps {
    () => {
        Oid!();
    };
}

macro_rules! impl_529 {
    () => {
        deps!();
        impl AsRef < [u8] > for Oid { fn as_ref (& self) -> & [u8] { self . as_bytes () } }
    };
}

impl_529!()