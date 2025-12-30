// Generated macro for impl_211 (impl)
macro_rules! Depcrate_bstrimpl_211 {
() => {
// Module: crate::bstr
// Provides: {"impl_211"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl IndexMut < RangeInclusive < usize > > for ByteString { # [inline] fn index_mut (& mut self , r : RangeInclusive < usize >) -> & mut ByteStr { ByteStr :: from_bytes_mut (& mut self . 0 [r]) } }
};
}
