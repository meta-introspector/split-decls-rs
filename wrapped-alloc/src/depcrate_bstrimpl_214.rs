// Generated macro for impl_214 (impl)
macro_rules! Depcrate_bstrimpl_214 {
() => {
// Module: crate::bstr
// Provides: {"impl_214"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl IndexMut < RangeToInclusive < usize > > for ByteString { # [inline] fn index_mut (& mut self , r : RangeToInclusive < usize >) -> & mut ByteStr { ByteStr :: from_bytes_mut (& mut self . 0 [r]) } }
};
}
