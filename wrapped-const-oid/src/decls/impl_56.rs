macro_rules! deps {
    () => {
        ObjectIdentifierRef!();
        ObjectIdentifier!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < const MAX_SIZE : usize > Borrow < ObjectIdentifierRef > for ObjectIdentifier < MAX_SIZE > { fn borrow (& self) -> & ObjectIdentifierRef { self . as_oid_ref () } }
    };
}

impl_56!()