// Generated macro for impl_209 (impl)
macro_rules! Depcrate_bstrimpl_209 {
() => {
// Module: crate::bstr
// Provides: {"impl_209"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl IndexMut < RangeFull > for ByteString { # [inline] fn index_mut (& mut self , _ : RangeFull) -> & mut ByteStr { self . as_mut_bytestr () } }
};
}
