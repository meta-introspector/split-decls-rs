macro_rules! deps {
    () => {
        ObjectId!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl Borrow < oid > for ObjectId { fn borrow (& self) -> & oid { self . as_ref () } }
    };
}

impl_20!();