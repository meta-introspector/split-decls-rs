macro_rules! deps {
    () => {
        ObjectIdentifierRef!();
        ObjectIdentifier!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < const MAX_SIZE : usize > PartialEq < ObjectIdentifier < MAX_SIZE > > for ObjectIdentifierRef { fn eq (& self , other : & ObjectIdentifier < MAX_SIZE >) -> bool { self . as_bytes () . eq (other . as_bytes ()) } }
    };
}

impl_71!()