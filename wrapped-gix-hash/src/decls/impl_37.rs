macro_rules! deps {
    () => {
        Error!();
        Kind!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl TryFrom < u8 > for Kind { type Error = u8 ; fn try_from (value : u8) -> Result < Self , Self :: Error > { Ok (match value { 1 => Kind :: Sha1 , unknown => return Err (unknown) , }) } }
    };
}

impl_37!();