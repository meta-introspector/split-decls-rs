macro_rules! deps {
    () => {
        UnknownCryptoError!();
        StreamTag!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl TryFrom < u8 > for StreamTag { type Error = UnknownCryptoError ; fn try_from (byte : u8) -> Result < Self , Self :: Error > { match byte { 0b0000_0000 => Ok (Self :: Message) , 0b0000_0001 => Ok (Self :: Push) , 0b0000_0010 => Ok (Self :: Rekey) , 0b0000_0011 => Ok (Self :: Finish) , _ => Err (UnknownCryptoError) , } } }
    };
}

impl_94!();