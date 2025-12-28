macro_rules! deps {
    () => {
        CursorType!();
        Result!();
        Error!();
    };
}

macro_rules! impl_713 {
    () => {
        deps!();
        impl CursorType for bool { type Error = ParseBoolError ; fn decode_cursor (s : & str) -> Result < Self , Self :: Error > { s . parse () } fn encode_cursor (& self) -> String { self . to_string () } }
    };
}

impl_713!()