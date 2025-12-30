// Generated macro for impl_968 (impl)
macro_rules! Depcrate_llvm_ffiimpl_968 {
() => {
// Module: crate::llvm::ffi
// Provides: {"impl_968"}
// Dependencies: {}
impl < T : TryFrom < u32 > > RawEnum < T > { # [track_caller] pub (crate) fn to_rust (self) -> T where T :: Error : Debug , { T :: try_from (self . value) . expect ("enum value returned by LLVM should be known") } }
};
}
