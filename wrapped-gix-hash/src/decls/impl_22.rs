macro_rules! deps {
    () => {
        ObjectId!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl PartialEq < & oid > for ObjectId { fn eq (& self , other : & & oid) -> bool { self . as_ref () == * other } }
    };
}

impl_22!()