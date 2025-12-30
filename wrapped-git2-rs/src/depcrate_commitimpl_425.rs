// Generated macro for impl_425 (impl)
macro_rules! Depcrate_commitimpl_425 {
() => {
// Module: crate::commit
// Provides: {"impl_425"}
// Dependencies: {}
impl < 'repo > Clone for Commit < 'repo > { fn clone (& self) -> Self { self . as_object () . clone () . into_commit () . ok () . unwrap () } }
};
}
