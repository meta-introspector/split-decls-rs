// Generated macro for hexdigit (function)
macro_rules! Depcrate_symbol_namehexdigit {
() => {
// Module: crate::symbol_name
// Provides: {"hexdigit"}
// Dependencies: {}
fn hexdigit (v : u32) -> char { if v < 10 { char :: from_u32 (('0' as u32) + v) . unwrap () } else { char :: from_u32 (('A' as u32) - 10 + v) . unwrap () } }
};
}
