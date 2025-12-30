// Generated macro for impl_207 (impl)
macro_rules! Depcrate_bstrimpl_207 {
() => {
// Module: crate::bstr
// Provides: {"impl_207"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl Index < RangeToInclusive < usize > > for ByteString { type Output = ByteStr ; # [inline] fn index (& self , r : RangeToInclusive < usize >) -> & ByteStr { ByteStr :: from_bytes (& self . 0 [r]) } }
};
}
