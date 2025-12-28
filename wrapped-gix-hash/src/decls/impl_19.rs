macro_rules! deps {
    () => {
        ObjectId!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl AsRef < oid > for ObjectId { fn as_ref (& self) -> & oid { oid :: from_bytes_unchecked (self . as_slice ()) } }
    };
}

impl_19!();