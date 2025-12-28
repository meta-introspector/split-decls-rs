macro_rules! deps {
    () => {
        ObjectIdentifier!();
        ObjectIdentifierRef!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < const MAX_SIZE : usize > PartialEq < ObjectIdentifier < MAX_SIZE > > for ObjectIdentifierRef { fn eq (& self , other : & ObjectIdentifier < MAX_SIZE >) -> bool { self . as_bytes () . eq (other . as_bytes ()) } }
    };
}

impl_29!()