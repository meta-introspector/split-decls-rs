// Generated macro for impl_153 (impl)
macro_rules! Depcrate_errorimpl_153 {
() => {
// Module: crate::error
// Provides: {"impl_153"}
// Dependencies: {}
impl < 'e > From < & 'e hir :: Error > for Formatter < 'e , hir :: ErrorKind > { fn from (err : & 'e hir :: Error) -> Self { Formatter { pattern : err . pattern () , err : err . kind () , span : err . span () , aux_span : None , } } }
};
}
