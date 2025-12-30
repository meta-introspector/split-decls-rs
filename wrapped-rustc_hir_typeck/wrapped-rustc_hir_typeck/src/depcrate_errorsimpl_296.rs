// Generated macro for impl_296 (impl)
macro_rules! Depcrate_errorsimpl_296 {
() => {
// Module: crate::errors
// Provides: {"impl_296"}
// Dependencies: {}
impl < G : EmissionGuarantee > Diagnostic < '_ , G > for NakedFunctionsAsmBlock { # [track_caller] fn into_diag (self , dcx : DiagCtxtHandle < '_ > , level : Level) -> Diag < '_ , G > { let mut diag = Diag :: new (dcx , level , fluent :: hir_typeck_naked_functions_asm_block) ; diag . span (self . span) ; diag . code (E0787) ; for span in self . multiple_asms . iter () { diag . span_label (* span , fluent :: hir_typeck_label_multiple_asm) ; } for span in self . non_asms . iter () { diag . span_label (* span , fluent :: hir_typeck_label_non_asm) ; } diag } }
};
}
