macro_rules! deps {
    () => {
        CursorType!();
        Error!();
        Result!();
    };
}

macro_rules! impl_717 {
    () => {
        deps!();
        # [cfg (feature = "uuid")] impl CursorType for uuid :: Uuid { type Error = uuid :: Error ; fn decode_cursor (s : & str) -> Result < Self , Self :: Error > { s . parse () } fn encode_cursor (& self) -> String { self . to_string () } }
    };
}

impl_717!()