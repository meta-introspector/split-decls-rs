// Generated macro for impl_39 (impl)
macro_rules! Depcrate_deimpl_39 {
() => {
// Module: crate::de
// Provides: {"impl_39"}
// Dependencies: {}
impl < 'a > Deserializer < read :: SliceRead < 'a > > { # [doc = " Creates a JSON deserializer from a `&[u8]`."] pub fn from_slice (bytes : & 'a [u8]) -> Self { Deserializer :: new (read :: SliceRead :: new (bytes)) } }
};
}
