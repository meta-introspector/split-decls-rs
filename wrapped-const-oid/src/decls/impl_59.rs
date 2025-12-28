macro_rules! deps {
    () => {
        Result!();
        ObjectIdentifier!();
        Error!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl TryFrom < & [u8] > for ObjectIdentifier { type Error = Error ; fn try_from (ber_bytes : & [u8]) -> Result < Self > { Self :: from_bytes (ber_bytes) } }
    };
}

impl_59!()