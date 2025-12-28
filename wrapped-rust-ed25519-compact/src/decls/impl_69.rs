macro_rules! deps {
    () => {
        Error!();
        Signature!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl TryFrom < & [u8] > for Signature { type Error = Error ; fn try_from (slice : & [u8]) -> Result < Self , Self :: Error > { Signature :: from_slice (slice) } }
    };
}

impl_69!()