// Generated macro for impl_205 (impl)
macro_rules! Depcrate_bstrimpl_205 {
() => {
// Module: crate::bstr
// Provides: {"impl_205"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl Index < RangeFrom < usize > > for ByteString { type Output = ByteStr ; # [inline] fn index (& self , r : RangeFrom < usize >) -> & ByteStr { ByteStr :: from_bytes (& self . 0 [r]) } }
};
}
