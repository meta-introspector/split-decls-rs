// Generated macro for impl_54 (impl)
macro_rules! Depcrate_extern_abiimpl_54 {
() => {
// Module: crate::extern_abi
// Provides: {"impl_54"}
// Dependencies: {}
impl ExternAbi { # [doc = " Default ABI chosen for `extern fn` declarations without an explicit ABI."] pub const FALLBACK : ExternAbi = ExternAbi :: C { unwind : false } ; pub fn name (self) -> & 'static str { self . as_str () } }
};
}
