// Generated macro for emit_macro_def_diagnostics (function)
macro_rules! Depcrateemit_macro_def_diagnostics {
() => {
// Module: crate
// Provides: {"emit_macro_def_diagnostics"}
// Dependencies: {}
fn emit_macro_def_diagnostics < 'db > (db : & 'db dyn HirDatabase , acc : & mut Vec < AnyDiagnostic < 'db > > , m : Macro ,) { let id = db . macro_def (m . id) ; if let hir_expand :: db :: TokenExpander :: DeclarativeMacro (expander) = db . macro_expander (id) && let Some (e) = expander . mac . err () { let Some (ast) = id . ast_id () . left () else { never ! ("declarative expander for non decl-macro: {:?}" , e) ; return ; } ; let krate = HasModule :: krate (& m . id , db) ; let edition = krate . data (db) . edition ; emit_def_diagnostic_ (db , acc , & DefDiagnosticKind :: MacroDefError { ast , message : e . to_string () } , edition ,) ; } }
};
}
