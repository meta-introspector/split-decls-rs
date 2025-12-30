// Generated macro for impl_409 (impl)
macro_rules! Depcrate_errorsimpl_409 {
() => {
// Module: crate::errors
// Provides: {"impl_409"}
// Dependencies: {}
impl Subdiagnostic for FormatUnusedArg { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { diag . arg ("named" , self . named) ; let msg = diag . eagerly_translate (crate :: fluent_generated :: builtin_macros_format_unused_arg) ; diag . remove_arg ("named") ; diag . span_label (self . span , msg) ; } }
};
}
