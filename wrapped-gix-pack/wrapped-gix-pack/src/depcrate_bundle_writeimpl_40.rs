// Generated macro for impl_40 (impl)
macro_rules! Depcrate_bundle_writeimpl_40 {
() => {
// Module: crate::bundle::write
// Provides: {"impl_40"}
// Dependencies: {}
impl From < ProgressId > for gix_features :: progress :: Id { fn from (v : ProgressId) -> Self { match v { ProgressId :: ReadPackBytes => * b"BWRB" , ProgressId :: IndexingSteps (_) => * b"BWCI" , } } }
};
}
