// Generated macro for impl_37 (impl)
macro_rules! Depcrate_decimalimpl_37 {
() => {
// Module: crate::decimal
// Provides: {"impl_37"}
// Dependencies: {}
impl FromStr for UnsignedDecimal { type Err = ParseError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: try_from_str (s) } }
};
}
