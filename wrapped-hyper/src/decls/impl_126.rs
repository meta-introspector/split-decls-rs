macro_rules! deps {
    () => {
        Error!();
        ReasonPhrase!();
        Result!();
        InvalidReasonPhrase!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl TryFrom < & [u8] > for ReasonPhrase { type Error = InvalidReasonPhrase ; fn try_from (reason : & [u8]) -> Result < Self , Self :: Error > { if let Some (bad_byte) = find_invalid_byte (reason) { Err (InvalidReasonPhrase { bad_byte }) } else { Ok (Self (Bytes :: copy_from_slice (reason))) } } }
    };
}

impl_126!()