macro_rules! deps {
    () => {
        ObjectIdentifier!();
        ObjectIdentifierRef!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < const MAX_SIZE : usize > Deref for ObjectIdentifier < MAX_SIZE > { type Target = ObjectIdentifierRef ; fn deref (& self) -> & ObjectIdentifierRef { self . as_oid_ref () } }
    };
}

impl_15!()