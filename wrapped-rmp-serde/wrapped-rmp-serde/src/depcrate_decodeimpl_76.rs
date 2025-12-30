// Generated macro for impl_76 (impl)
macro_rules! Depcrate_decodeimpl_76 {
() => {
// Module: crate::decode
// Provides: {"impl_76"}
// Dependencies: {}
impl < R : Read , C > Deserializer < ReadReader < R > , C > { # [doc = " Gets a reference to the underlying reader in this decoder."] # [inline (always)] pub fn get_ref (& self) -> & R { & self . rd . rd } # [doc = " Gets a mutable reference to the underlying reader in this decoder."] # [inline (always)] pub fn get_mut (& mut self) -> & mut R { & mut self . rd . rd } # [doc = " Consumes this deserializer returning the underlying reader."] # [inline] pub fn into_inner (self) -> R { self . rd . rd } }
};
}
