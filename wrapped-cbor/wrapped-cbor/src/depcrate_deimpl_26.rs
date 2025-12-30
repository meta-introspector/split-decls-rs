// Generated macro for impl_26 (impl)
macro_rules! Depcrate_deimpl_26 {
() => {
// Module: crate::de
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'a > Deserializer < MutSliceRead < 'a > > { # [doc = " Constructs a `Deserializer` which reads from a mutable slice that doubles as its own"] # [doc = " scratch buffer."] # [doc = ""] # [doc = " Borrowed strings and byte slices will be provided even for indefinite strings."] pub fn from_mut_slice (bytes : & 'a mut [u8]) -> Deserializer < MutSliceRead < 'a > > { Deserializer :: new (MutSliceRead :: new (bytes)) } }
};
}
