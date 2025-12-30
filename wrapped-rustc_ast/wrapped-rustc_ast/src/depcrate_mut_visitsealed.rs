// Generated macro for sealed (module)
macro_rules! Depcrate_mut_visitsealed {
() => {
// Module: crate::mut_visit
// Provides: {"sealed"}
// Dependencies: {}
mod sealed { use rustc_ast_ir :: visit :: VisitorResult ; # [doc = " This is for compatibility with the regular `Visitor`."] pub trait MutVisitorResult { type Result : VisitorResult ; } impl < T > MutVisitorResult for T { type Result = () ; } }
};
}
