// Generated macro for impl_81 (impl)
macro_rules! Depcrate_charimpl_81 {
() => {
// Module: crate::char
// Provides: {"impl_81"}
// Dependencies: {}
impl CharLit < & str > { # [doc = " Makes a copy of the underlying buffer and returns the owned version of"] # [doc = " `Self`."] pub fn to_owned (& self) -> CharLit < String > { CharLit { raw : self . raw . to_owned () , start_suffix : self . start_suffix , value : self . value , } } }
};
}
