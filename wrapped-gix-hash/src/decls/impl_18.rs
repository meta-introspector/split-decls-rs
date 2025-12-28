macro_rules! deps {
    () => {
        ObjectId!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl Deref for ObjectId { type Target = oid ; fn deref (& self) -> & Self :: Target { self . as_ref () } }
    };
}

impl_18!()