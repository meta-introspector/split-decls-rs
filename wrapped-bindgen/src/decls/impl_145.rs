macro_rules! deps {
    () => {
        Literal!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl Literal { unsuffixed ! (i64 => i64_unsuffixed) ; unsuffixed ! (usize => usize_unsuffixed) ; unsuffixed ! (u32 => u32_unsuffixed) ; unsuffixed ! (u16 => u16_unsuffixed) ; unsuffixed ! (u8 => u8_unsuffixed) ; pub fn byte_string (s : & str) -> Self { Self { inner : format ! ("b\"{s}\"") , } } pub fn as_str (& self) -> & str { & self . inner } }
    };
}

impl_145!();