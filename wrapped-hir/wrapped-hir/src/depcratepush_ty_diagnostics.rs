// Generated macro for push_ty_diagnostics (function)
macro_rules! Depcratepush_ty_diagnostics {
() => {
// Module: crate
// Provides: {"push_ty_diagnostics"}
// Dependencies: {}
fn push_ty_diagnostics < 'db > (db : & 'db dyn HirDatabase , acc : & mut Vec < AnyDiagnostic < 'db > > , diagnostics : Option < ThinArc < () , TyLoweringDiagnostic > > , source_map : & ExpressionStoreSourceMap ,) { if let Some (diagnostics) = diagnostics { acc . extend (diagnostics . slice . iter () . filter_map (| diagnostic | AnyDiagnostic :: ty_diagnostic (diagnostic , source_map , db)) ,) ; } }
};
}
