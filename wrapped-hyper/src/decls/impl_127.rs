macro_rules! deps {
    () => {
        Result!();
        InvalidReasonPhrase!();
        Error!();
        ReasonPhrase!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl TryFrom < Vec < u8 > > for ReasonPhrase { type Error = InvalidReasonPhrase ; fn try_from (reason : Vec < u8 >) -> Result < Self , Self :: Error > { if let Some (bad_byte) = find_invalid_byte (& reason) { Err (InvalidReasonPhrase { bad_byte }) } else { Ok (Self (Bytes :: from (reason))) } } }
    };
}

impl_127!()