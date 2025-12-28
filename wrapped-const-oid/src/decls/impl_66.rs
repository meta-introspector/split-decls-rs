macro_rules! deps {
    () => {
        ObjectIdentifierRef!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl AsRef < [u8] > for ObjectIdentifierRef { fn as_ref (& self) -> & [u8] { self . as_bytes () } }
    };
}

impl_66!();