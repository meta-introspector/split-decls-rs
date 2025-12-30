// Generated macro for impl_129 (impl)
macro_rules! Depcrate_ntstatusimpl_129 {
() => {
// Module: crate::ntstatus
// Provides: {"impl_129"}
// Dependencies: {}
impl From < NTSTATUS > for Error { fn from (value : NTSTATUS) -> Self { value . to_hresult () . into () } }
};
}
