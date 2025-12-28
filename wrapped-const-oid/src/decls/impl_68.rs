macro_rules! deps {
    () => {
        ObjectIdentifierRef!();
        Result!();
        Error!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < 'a > TryFrom < & 'a [u8] > for & 'a ObjectIdentifierRef { type Error = Error ; fn try_from (ber_bytes : & 'a [u8]) -> Result < Self > { ObjectIdentifierRef :: from_bytes (ber_bytes) } }
    };
}

impl_68!()