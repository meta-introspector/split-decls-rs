// Generated macro for impl_206 (impl)
macro_rules! Depcrate_bstrimpl_206 {
() => {
// Module: crate::bstr
// Provides: {"impl_206"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl Index < RangeTo < usize > > for ByteString { type Output = ByteStr ; # [inline] fn index (& self , r : RangeTo < usize >) -> & ByteStr { ByteStr :: from_bytes (& self . 0 [r]) } }
};
}
