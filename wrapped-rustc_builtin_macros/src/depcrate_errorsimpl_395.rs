// Generated macro for impl_395 (impl)
macro_rules! Depcrate_errorsimpl_395 {
() => {
// Module: crate::errors
// Provides: {"impl_395"}
// Dependencies: {}
impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for EnvNotDefinedWithUserMessage { # [track_caller] fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { # [expect (rustc :: untranslatable_diagnostic , reason = "cannot translate user-provided messages")] let mut diag = Diag :: new (dcx , level , self . msg_from_user . to_string ()) ; diag . span (self . span) ; diag } }
};
}
