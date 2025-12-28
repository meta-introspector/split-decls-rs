macro_rules! deps {
    () => {
        ObjectIdentifierRef!();
        ObjectIdentifier!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < const MAX_SIZE : usize > AsRef < ObjectIdentifierRef > for ObjectIdentifier < MAX_SIZE > { fn as_ref (& self) -> & ObjectIdentifierRef { self . as_oid_ref () } }
    };
}

impl_55!()