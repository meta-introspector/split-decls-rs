macro_rules! deps {
    () => {
        ObjectIdentifierRef!();
        ObjectIdentifier!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < 'a , const MAX_SIZE : usize > From < & 'a ObjectIdentifier < MAX_SIZE > > for & 'a ObjectIdentifierRef { fn from (oid : & 'a ObjectIdentifier < MAX_SIZE >) -> & 'a ObjectIdentifierRef { oid . as_oid_ref () } }
    };
}

impl_25!()