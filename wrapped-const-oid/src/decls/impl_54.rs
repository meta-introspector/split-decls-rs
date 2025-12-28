macro_rules! deps {
    () => {
        ObjectIdentifier!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < const MAX_SIZE : usize > AsRef < [u8] > for ObjectIdentifier < MAX_SIZE > { fn as_ref (& self) -> & [u8] { self . as_bytes () } }
    };
}

impl_54!()