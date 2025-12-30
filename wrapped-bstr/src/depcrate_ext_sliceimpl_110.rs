// Generated macro for impl_110 (impl)
macro_rules! Depcrate_ext_sliceimpl_110 {
() => {
// Module: crate::ext_slice
// Provides: {"impl_110"}
// Dependencies: {}
impl < 'a , F : FnMut (char) -> bool > FieldsWith < 'a , F > { fn new (bytes : & 'a [u8] , f : F) -> FieldsWith < 'a , F > { FieldsWith { f , bytes , chars : bytes . char_indices () } } }
};
}
