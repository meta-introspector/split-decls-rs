// Generated macro for impl_262 (impl)
macro_rules! Depcrate_stringimpl_262 {
() => {
// Module: crate::string
// Provides: {"impl_262"}
// Dependencies: {}
impl StringLit < & str > { # [doc = " Makes a copy of the underlying buffer and returns the owned version of"] # [doc = " `Self`."] pub fn into_owned (self) -> StringLit < String > { StringLit { raw : self . raw . to_owned () , value : self . value , num_hashes : self . num_hashes , start_suffix : self . start_suffix , } } }
};
}
