// Generated macro for impl_18 (impl)
macro_rules! Depcrate_wasmimpl_18 {
() => {
// Module: crate::wasm
// Provides: {"impl_18"}
// Dependencies: {}
impl Wasmi32Ptr { pub (crate) fn as_usize (& self) -> usize { let Val :: I32 (val) = self . 0 else { unreachable ! () } ; val . try_into () . unwrap () } }
};
}
