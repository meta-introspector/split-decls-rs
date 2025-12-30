// Generated macro for impl_119 (impl)
macro_rules! Depcrate_signed_decimalimpl_119 {
() => {
// Module: crate::signed_decimal
// Provides: {"impl_119"}
// Dependencies: {}
impl FromStr for Decimal { type Err = ParseError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: try_from_str (s) } }
};
}
