// Generated macro for impl_358 (impl)
macro_rules! Depcrate_blobimpl_358 {
() => {
// Module: crate::blob
// Provides: {"impl_358"}
// Dependencies: {}
impl < 'repo > Clone for Blob < 'repo > { fn clone (& self) -> Self { self . as_object () . clone () . into_blob () . ok () . unwrap () } }
};
}
