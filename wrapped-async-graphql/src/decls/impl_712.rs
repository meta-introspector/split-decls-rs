macro_rules! deps {
    () => {
        Error!();
        CursorType!();
        Result!();
    };
}

macro_rules! impl_712 {
    () => {
        deps!();
        impl CursorType for char { type Error = ParseCharError ; fn decode_cursor (s : & str) -> Result < Self , Self :: Error > { s . parse () } fn encode_cursor (& self) -> String { self . to_string () } }
    };
}

impl_712!();