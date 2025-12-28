macro_rules! deps {
    () => {
        ObjectIdentifier!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl TryFrom < & [u8] > for ObjectIdentifier { type Error = Error ; fn try_from (ber_bytes : & [u8]) -> Result < Self > { Self :: from_bytes (ber_bytes) } }
    };
}

impl_17!()