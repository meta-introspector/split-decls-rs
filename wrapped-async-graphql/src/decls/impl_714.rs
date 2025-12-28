macro_rules! deps {
    () => {
        CursorType!();
        Error!();
        Result!();
    };
}

macro_rules! impl_714 {
    () => {
        deps!();
        impl CursorType for String { type Error = Infallible ; fn decode_cursor (s : & str) -> Result < Self , Self :: Error > { Ok (s . to_string ()) } fn encode_cursor (& self) -> String { self . clone () } }
    };
}

impl_714!()