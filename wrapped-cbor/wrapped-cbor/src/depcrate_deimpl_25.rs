// Generated macro for impl_25 (impl)
macro_rules! Depcrate_deimpl_25 {
() => {
// Module: crate::de
// Provides: {"impl_25"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'a > Deserializer < SliceRead < 'a > > { # [doc = " Constructs a `Deserializer` which reads from a slice."] # [doc = ""] # [doc = " Borrowed strings and byte slices will be provided when possible."] pub fn from_slice (bytes : & 'a [u8]) -> Deserializer < SliceRead < 'a > > { Deserializer :: new (SliceRead :: new (bytes)) } }
};
}
