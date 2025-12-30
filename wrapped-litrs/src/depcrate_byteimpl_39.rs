// Generated macro for impl_39 (impl)
macro_rules! Depcrate_byteimpl_39 {
() => {
// Module: crate::byte
// Provides: {"impl_39"}
// Dependencies: {}
impl ByteLit < & str > { # [doc = " Makes a copy of the underlying buffer and returns the owned version of"] # [doc = " `Self`."] pub fn to_owned (& self) -> ByteLit < String > { ByteLit { raw : self . raw . to_owned () , start_suffix : self . start_suffix , value : self . value , } } }
};
}
