// Generated macro for impl_522 (impl)
macro_rules! Depcrate_argimpl_522 {
() => {
// Module: crate::arg
// Provides: {"impl_522"}
// Dependencies: {}
impl ArgType { # [doc = " A str corresponding to the name of a Rust type."] pub fn as_str (self) -> & 'static str { ALL_ARG_TYPES . iter () . skip_while (| a | a . 0 != self) . next () . unwrap () . 1 } # [doc = " Returns a Vec of all possible argtypes."] pub fn all () -> Vec < Self > { ALL_ARG_TYPES . iter () . map (| x | x . 0) . collect () } # [doc = " Converts an i32 to an ArgType (or an error)."] pub fn from_i32 (i : i32) -> Result < ArgType , String > { for & (a , _) in & ALL_ARG_TYPES { if a as i32 == i { return Ok (a) ; } } Err (format ! ("Invalid ArgType {} ({})" , i , i as u8 as char)) } }
};
}
