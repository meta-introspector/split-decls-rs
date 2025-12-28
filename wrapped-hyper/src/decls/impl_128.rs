macro_rules! deps {
    () => {
        InvalidReasonPhrase!();
        Error!();
        Result!();
        ReasonPhrase!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl TryFrom < String > for ReasonPhrase { type Error = InvalidReasonPhrase ; fn try_from (reason : String) -> Result < Self , Self :: Error > { if let Some (bad_byte) = find_invalid_byte (reason . as_bytes ()) { Err (InvalidReasonPhrase { bad_byte }) } else { Ok (Self (Bytes :: from (reason))) } } }
    };
}

impl_128!()