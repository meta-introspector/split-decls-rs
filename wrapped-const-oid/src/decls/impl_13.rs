macro_rules! deps {
    () => {
        ObjectIdentifier!();
        ObjectIdentifierRef!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < const MAX_SIZE : usize > AsRef < ObjectIdentifierRef > for ObjectIdentifier < MAX_SIZE > { fn as_ref (& self) -> & ObjectIdentifierRef { self . as_oid_ref () } }
    };
}

impl_13!()