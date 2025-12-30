// Generated macro for impl_59 (impl)
macro_rules! Depcrate_bytestrimpl_59 {
() => {
// Module: crate::bytestr
// Provides: {"impl_59"}
// Dependencies: {}
impl ByteStringLit < & str > { # [doc = " Makes a copy of the underlying buffer and returns the owned version of"] # [doc = " `Self`."] pub fn into_owned (self) -> ByteStringLit < String > { ByteStringLit { raw : self . raw . to_owned () , value : self . value , num_hashes : self . num_hashes , start_suffix : self . start_suffix , } } }
};
}
