macro_rules! deps {
    () => {
        Result!();
        CursorType!();
        Error!();
    };
}

macro_rules! impl_710 {
    () => {
        deps!();
        impl CursorType for f32 { type Error = ParseFloatError ; fn decode_cursor (s : & str) -> Result < Self , Self :: Error > { s . parse () } fn encode_cursor (& self) -> String { self . to_string () } }
    };
}

impl_710!();