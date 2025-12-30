// Generated macro for impl_105 (impl)
macro_rules! Depcrate_cstrimpl_105 {
() => {
// Module: crate::cstr
// Provides: {"impl_105"}
// Dependencies: {}
impl CStringLit < & str > { # [doc = " Makes a copy of the underlying buffer and returns the owned version of"] # [doc = " `Self`."] pub fn into_owned (self) -> CStringLit < String > { CStringLit { raw : self . raw . to_owned () , value : self . value , num_hashes : self . num_hashes , start_suffix : self . start_suffix , } } }
};
}
