// Generated macro for impl_78 (impl)
macro_rules! Depcrate_decodeimpl_78 {
() => {
// Module: crate::decode
// Provides: {"impl_78"}
// Dependencies: {}
impl < R : AsRef < [u8] > > Deserializer < ReadReader < Cursor < R > > > { # [doc = " Returns the current position of this deserializer, i.e. how many bytes were read."] # [inline (always)] pub fn position (& self) -> u64 { self . rd . rd . position () } }
};
}
