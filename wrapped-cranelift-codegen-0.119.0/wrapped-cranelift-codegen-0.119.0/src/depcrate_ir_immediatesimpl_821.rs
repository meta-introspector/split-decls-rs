// Generated macro for impl_821 (impl)
macro_rules! Depcrate_ir_immediatesimpl_821 {
() => {
// Module: crate::ir::immediates
// Provides: {"impl_821"}
// Dependencies: {}
impl FromStr for Uimm32 { type Err = & 'static str ; fn from_str (s : & str) -> Result < Self , & 'static str > { parse_i64 (s) . and_then (| x | { if 0 <= x && x <= i64 :: from (u32 :: MAX) { Ok (Self (x as u32)) } else { Err ("Uimm32 out of range") } }) } }
};
}
