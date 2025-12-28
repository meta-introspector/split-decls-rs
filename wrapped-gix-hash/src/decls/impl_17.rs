macro_rules! deps {
    () => {
        ObjectId!();
        Error!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl TryFrom < & [u8] > for ObjectId { type Error = crate :: Error ; fn try_from (bytes : & [u8]) -> Result < Self , Self :: Error > { Ok (oid :: try_from_bytes (bytes) ? . into ()) } }
    };
}

impl_17!();