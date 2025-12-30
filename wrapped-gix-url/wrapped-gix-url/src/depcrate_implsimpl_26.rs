// Generated macro for impl_26 (impl)
macro_rules! Depcrate_implsimpl_26 {
() => {
// Module: crate::impls
// Provides: {"impl_26"}
// Dependencies: {}
impl TryFrom < PathBuf > for Url { type Error = parse :: Error ; fn try_from (value : PathBuf) -> Result < Self , Self :: Error > { gix_path :: into_bstr (value) . try_into () } }
};
}
