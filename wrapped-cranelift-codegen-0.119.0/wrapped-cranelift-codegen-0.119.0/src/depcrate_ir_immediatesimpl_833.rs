// Generated macro for impl_833 (impl)
macro_rules! Depcrate_ir_immediatesimpl_833 {
() => {
// Module: crate::ir::immediates
// Provides: {"impl_833"}
// Dependencies: {}
impl FromStr for Offset32 { type Err = & 'static str ; fn from_str (s : & str) -> Result < Self , & 'static str > { if ! (s . starts_with ('-') || s . starts_with ('+')) { return Err ("Offset must begin with sign") ; } parse_i64 (s) . and_then (| x | { if i64 :: from (i32 :: MIN) <= x && x <= i64 :: from (i32 :: MAX) { Ok (Self :: new (x as i32)) } else { Err ("Offset out of range") } }) } }
};
}
