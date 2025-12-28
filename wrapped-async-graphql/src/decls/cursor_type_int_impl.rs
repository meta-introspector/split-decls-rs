macro_rules! deps {
    () => {
        Error!();
        Result!();
        CursorType!();
    };
}

macro_rules! cursor_type_int_impl {
    () => {
        deps!();
        macro_rules ! cursor_type_int_impl { ($ ($ t : ty) *) => { $ (impl CursorType for $ t { type Error = ParseIntError ; fn decode_cursor (s : & str) -> Result < Self , Self :: Error > { s . parse () } fn encode_cursor (& self) -> String { self . to_string () } }) * } }
    };
}

cursor_type_int_impl!()