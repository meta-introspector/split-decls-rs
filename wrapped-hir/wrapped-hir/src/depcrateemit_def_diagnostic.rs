// Generated macro for emit_def_diagnostic (function)
macro_rules! Depcrateemit_def_diagnostic {
() => {
// Module: crate
// Provides: {"emit_def_diagnostic"}
// Dependencies: {}
fn emit_def_diagnostic < 'db > (db : & 'db dyn HirDatabase , acc : & mut Vec < AnyDiagnostic < 'db > > , diag : & DefDiagnostic , edition : Edition ,) { emit_def_diagnostic_ (db , acc , & diag . kind , edition) }
};
}
