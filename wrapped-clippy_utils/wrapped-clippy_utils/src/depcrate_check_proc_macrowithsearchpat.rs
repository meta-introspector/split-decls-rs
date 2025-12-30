// Generated macro for WithSearchPat (trait)
macro_rules! Depcrate_check_proc_macroWithSearchPat {
() => {
// Module: crate::check_proc_macro
// Provides: {"WithSearchPat"}
// Dependencies: {}
pub trait WithSearchPat < 'cx > { type Context : LintContext ; fn search_pat (& self , cx : & Self :: Context) -> (Pat , Pat) ; fn span (& self) -> Span ; }
};
}
