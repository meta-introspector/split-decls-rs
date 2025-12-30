// Generated macro for impl_203 (impl)
macro_rules! Depcrate_bstrimpl_203 {
() => {
// Module: crate::bstr
// Provides: {"impl_203"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl Index < Range < usize > > for ByteString { type Output = ByteStr ; # [inline] fn index (& self , r : Range < usize >) -> & ByteStr { ByteStr :: from_bytes (& self . 0 [r]) } }
};
}
