macro_rules! deps {
    () => {
        Result!();
        InvalidReasonPhrase!();
        Error!();
        ReasonPhrase!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl TryFrom < Bytes > for ReasonPhrase { type Error = InvalidReasonPhrase ; fn try_from (reason : Bytes) -> Result < Self , Self :: Error > { if let Some (bad_byte) = find_invalid_byte (& reason) { Err (InvalidReasonPhrase { bad_byte }) } else { Ok (Self (reason)) } } }
    };
}

impl_129!()