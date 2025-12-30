// Generated macro for impl_42 (impl)
macro_rules! Depcrate_punycodeimpl_42 {
() => {
// Module: crate::punycode
// Provides: {"impl_42"}
// Dependencies: {}
impl PunycodeCodeUnit for char { fn is_delimiter (& self) -> bool { * self == '-' } fn is_ascii (& self) -> bool { debug_assert ! (false) ; true } fn digit (& self) -> Option < u32 > { let byte = * self ; Some (match byte { byte @ '0' ..= '9' => u32 :: from (byte) - u32 :: from ('0') + 26 , byte @ 'a' ..= 'z' => u32 :: from (byte) - u32 :: from ('a') , _ => return None , }) } fn char (& self) -> char { debug_assert ! (false) ; * self } fn char_ascii_lower_case (& self) -> char { * self } }
};
}
