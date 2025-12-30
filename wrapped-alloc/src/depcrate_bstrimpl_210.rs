// Generated macro for impl_210 (impl)
macro_rules! Depcrate_bstrimpl_210 {
() => {
// Module: crate::bstr
// Provides: {"impl_210"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl IndexMut < Range < usize > > for ByteString { # [inline] fn index_mut (& mut self , r : Range < usize >) -> & mut ByteStr { ByteStr :: from_bytes_mut (& mut self . 0 [r]) } }
};
}
