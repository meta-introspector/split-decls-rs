// Generated macro for impl_152 (impl)
macro_rules! Depcrate_errorimpl_152 {
() => {
// Module: crate::error
// Provides: {"impl_152"}
// Dependencies: {}
impl < 'e > From < & 'e ast :: Error > for Formatter < 'e , ast :: ErrorKind > { fn from (err : & 'e ast :: Error) -> Self { Formatter { pattern : err . pattern () , err : err . kind () , span : err . span () , aux_span : err . auxiliary_span () , } } }
};
}
