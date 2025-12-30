// Generated macro for impl_212 (impl)
macro_rules! Depcrate_bstrimpl_212 {
() => {
// Module: crate::bstr
// Provides: {"impl_212"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl IndexMut < RangeFrom < usize > > for ByteString { # [inline] fn index_mut (& mut self , r : RangeFrom < usize >) -> & mut ByteStr { ByteStr :: from_bytes_mut (& mut self . 0 [r]) } }
};
}
