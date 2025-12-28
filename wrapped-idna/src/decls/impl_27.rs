macro_rules! deps {
    () => {
        PunycodeCodeUnit!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl PunycodeCodeUnit for u8 { fn is_delimiter (& self) -> bool { * self == b'-' } fn is_ascii (& self) -> bool { * self < 0x80 } fn digit (& self) -> Option < u32 > { let byte = * self ; Some (match byte { byte @ b'0' ..= b'9' => byte - b'0' + 26 , byte @ b'A' ..= b'Z' => byte - b'A' , byte @ b'a' ..= b'z' => byte - b'a' , _ => return None , } as u32) } fn char (& self) -> char { char :: from (* self) } fn char_ascii_lower_case (& self) -> char { char :: from (self . to_ascii_lowercase ()) } }
    };
}

impl_27!()