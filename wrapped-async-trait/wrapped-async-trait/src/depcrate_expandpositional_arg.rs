// Generated macro for positional_arg (function)
macro_rules! Depcrate_expandpositional_arg {
() => {
// Module: crate::expand
// Provides: {"positional_arg"}
// Dependencies: {}
fn positional_arg (i : usize , pat : & Pat) -> Ident { let span = syn :: spanned :: Spanned :: span (pat) . resolved_at (Span :: mixed_site ()) ; format_ident ! ("__arg{}" , i , span = span) }
};
}
