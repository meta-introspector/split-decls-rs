macro_rules! deps {
    () => {
        PunycodeCodeUnit!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl PunycodeCodeUnit for char { fn is_delimiter (& self) -> bool { * self == '-' } fn is_ascii (& self) -> bool { debug_assert ! (false) ; true } fn digit (& self) -> Option < u32 > { let byte = * self ; Some (match byte { byte @ '0' ..= '9' => u32 :: from (byte) - u32 :: from ('0') + 26 , byte @ 'a' ..= 'z' => u32 :: from (byte) - u32 :: from ('a') , _ => return None , }) } fn char (& self) -> char { debug_assert ! (false) ; * self } fn char_ascii_lower_case (& self) -> char { * self } }
    };
}

impl_28!();