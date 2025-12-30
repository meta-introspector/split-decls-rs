// Generated macro for impl_204 (impl)
macro_rules! Depcrate_bstrimpl_204 {
() => {
// Module: crate::bstr
// Provides: {"impl_204"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl Index < RangeInclusive < usize > > for ByteString { type Output = ByteStr ; # [inline] fn index (& self , r : RangeInclusive < usize >) -> & ByteStr { ByteStr :: from_bytes (& self . 0 [r]) } }
};
}
