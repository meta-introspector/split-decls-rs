// Generated macro for impl_717 (impl)
macro_rules! Depcrate_lintsimpl_717 {
() => {
// Module: crate::lints
// Provides: {"impl_717"}
// Dependencies: {}
impl Subdiagnostic for BuiltinUnpermittedTypeInitSub { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { let mut err = self . err ; loop { if let Some (span) = err . span { diag . span_note (span , err . message) ; } else { diag . note (err . message) ; } if let Some (e) = err . nested { err = * e ; } else { break ; } } } }
};
}
