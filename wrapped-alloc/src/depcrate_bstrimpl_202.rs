// Generated macro for impl_202 (impl)
macro_rules! Depcrate_bstrimpl_202 {
() => {
// Module: crate::bstr
// Provides: {"impl_202"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl Index < RangeFull > for ByteString { type Output = ByteStr ; # [inline] fn index (& self , _ : RangeFull) -> & ByteStr { self . as_bytestr () } }
};
}
