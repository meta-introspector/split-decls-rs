macro_rules! deps {
    () => {
        RecoveryId!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl TryFrom < u8 > for RecoveryId { type Error = Error ; fn try_from (byte : u8) -> Result < Self > { Self :: from_byte (byte) . ok_or_else (Error :: new) } }
    };
}

impl_4!()