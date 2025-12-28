macro_rules! deps {
    () => {
        CursorType!();
        Result!();
        ID!();
        Error!();
    };
}

macro_rules! impl_715 {
    () => {
        deps!();
        impl CursorType for ID { type Error = Infallible ; fn decode_cursor (s : & str) -> Result < Self , Self :: Error > { Ok (s . to_string () . into ()) } fn encode_cursor (& self) -> String { self . to_string () } }
    };
}

impl_715!();