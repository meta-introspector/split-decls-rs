// Generated macro for impl_201 (impl)
macro_rules! Depcrate_bstrimpl_201 {
() => {
// Module: crate::bstr
// Provides: {"impl_201"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl Index < usize > for ByteString { type Output = u8 ; # [inline] fn index (& self , idx : usize) -> & u8 { & self . 0 [idx] } }
};
}
