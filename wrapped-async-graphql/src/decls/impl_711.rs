macro_rules! deps {
    () => {
        CursorType!();
        Error!();
        Result!();
    };
}

macro_rules! impl_711 {
    () => {
        deps!();
        impl CursorType for f64 { type Error = ParseFloatError ; fn decode_cursor (s : & str) -> Result < Self , Self :: Error > { s . parse () } fn encode_cursor (& self) -> String { self . to_string () } }
    };
}

impl_711!()