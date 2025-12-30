// Generated macro for make_external_component (function)
macro_rules! Depcrate_symbol_namemake_external_component {
() => {
// Module: crate::symbol_name
// Provides: {"make_external_component"}
// Dependencies: {}
# [doc = " encode symbol as alphanumeric by hex-encoding special characters"] pub fn make_external_component (input : & str) -> String { input . chars () . map (| c | match c { 'A' ..= 'Z' | 'a' ..= 'z' | '0' ..= '9' | '_' => { let mut s = String :: new () ; s . push (c) ; s } '-' => { let mut s = String :: new () ; s . push ('_') ; s } _ => { let mut s = String :: from ("X") ; s . push (hexdigit ((c as u32 & 0xf0) >> 4)) ; s . push (hexdigit (c as u32 & 0xf)) ; s } }) . collect () }
};
}
