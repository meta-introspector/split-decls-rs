macro_rules! deps {
    () => {
        DynAssociatedOid!();
        ObjectIdentifier!();
        AssociatedOid!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < T : AssociatedOid > DynAssociatedOid for T { fn oid (& self) -> ObjectIdentifier { T :: OID } }
    };
}

impl_40!()