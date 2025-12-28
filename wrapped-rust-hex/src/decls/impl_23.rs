macro_rules! deps {
    () => {
        FromHexError!();
        FromHex!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl FromHex for Vec < u8 > { type Error = FromHexError ; fn from_hex < T : AsRef < [u8] > > (hex : T) -> Result < Self , Self :: Error > { let hex = hex . as_ref () ; if hex . len () % 2 != 0 { return Err (FromHexError :: OddLength) ; } let mut out = vec ! [0 ; hex . len () / 2] ; decode_to_slice (hex , & mut out) ? ; Ok (out) } }
    };
}

impl_23!();