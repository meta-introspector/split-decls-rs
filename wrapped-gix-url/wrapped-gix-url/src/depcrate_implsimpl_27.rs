// Generated macro for impl_27 (impl)
macro_rules! Depcrate_implsimpl_27 {
() => {
// Module: crate::impls
// Provides: {"impl_27"}
// Dependencies: {}
impl TryFrom < & Path > for Url { type Error = parse :: Error ; fn try_from (value : & Path) -> Result < Self , Self :: Error > { gix_path :: into_bstr (value) . try_into () } }
};
}
