// Generated macro for impl_103 (impl)
macro_rules! Depcrate_cstrimpl_103 {
() => {
// Module: crate::cstr
// Provides: {"impl_103"}
// Dependencies: {}
impl < B : Buffer > CStringLit < B > { # [doc = " Parses the input as a (raw) byte string literal. Returns an error if the"] # [doc = " input is invalid or represents a different kind of literal."] pub fn parse (input : B) -> Result < Self , ParseError > { if input . is_empty () { return Err (perr (None , Empty)) ; } if ! input . starts_with (r#"c""#) && ! input . starts_with ("cr") { return Err (perr (None , InvalidCStringLiteralStart)) ; } let (value , num_hashes , start_suffix) = parse_impl (& input) ? ; Ok (Self { raw : input , value , num_hashes , start_suffix }) } # [doc = " Returns the string value this literal represents (where all escapes have"] # [doc = " been turned into their respective values)."] pub fn value (& self) -> & CStr { & self . value } # [doc = " Like `value` but returns an owned version of the value."] pub fn into_value (self) -> CString { self . value } # [doc = " The optional suffix. Returns `\"\"` if the suffix is empty/does not exist."] pub fn suffix (& self) -> & str { & (* self . raw) [self . start_suffix ..] } # [doc = " Returns whether this literal is a raw string literal (starting with"] # [doc = " `cr`)."] pub fn is_raw_c_string (& self) -> bool { self . num_hashes . is_some () } # [doc = " Returns the raw input that was passed to `parse`."] pub fn raw_input (& self) -> & str { & self . raw } # [doc = " Returns the raw input that was passed to `parse`, potentially owned."] pub fn into_raw_input (self) -> B { self . raw } }
};
}
