// Generated macro for impl_190 (impl)
macro_rules! Depcrate_refs_castimpl_190 {
() => {
// Module: crate::refs::cast
// Provides: {"impl_190"}
// Dependencies: {}
impl < 'local , 'from , To : Reference > Deref for Cast < 'local , 'from , To > { type Target = To :: Kind < 'local > ; fn deref (& self) -> & Self :: Target { & self . to } }
};
}
