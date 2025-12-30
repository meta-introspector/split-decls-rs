// Generated macro for impl_213 (impl)
macro_rules! Depcrate_bstrimpl_213 {
() => {
// Module: crate::bstr
// Provides: {"impl_213"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl IndexMut < RangeTo < usize > > for ByteString { # [inline] fn index_mut (& mut self , r : RangeTo < usize >) -> & mut ByteStr { ByteStr :: from_bytes_mut (& mut self . 0 [r]) } }
};
}
