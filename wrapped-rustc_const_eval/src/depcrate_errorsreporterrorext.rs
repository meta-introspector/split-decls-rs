// Generated macro for ReportErrorExt (trait)
macro_rules! Depcrate_errorsReportErrorExt {
() => {
// Module: crate::errors
// Provides: {"ReportErrorExt"}
// Dependencies: {}
pub trait ReportErrorExt { # [doc = " Returns the diagnostic message for this error."] fn diagnostic_message (& self) -> DiagMessage ; fn add_args < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) ; fn debug (self) -> String where Self : Sized , { ty :: tls :: with (move | tcx | { let dcx = tcx . dcx () ; let mut diag = dcx . struct_allow (DiagMessage :: Str (String :: new () . into ())) ; let message = self . diagnostic_message () ; self . add_args (& mut diag) ; let s = dcx . eagerly_translate_to_string (message , diag . args . iter ()) ; diag . cancel () ; s }) } }
};
}
