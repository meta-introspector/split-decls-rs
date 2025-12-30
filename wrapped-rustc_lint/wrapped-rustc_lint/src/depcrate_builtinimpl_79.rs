// Generated macro for impl_79 (impl)
macro_rules! Depcrate_builtinimpl_79 {
() => {
// Module: crate::builtin
// Provides: {"impl_79"}
// Dependencies: {}
impl UnsafeCode { fn report_unsafe (& self , cx : & EarlyContext < '_ > , span : Span , decorate : impl for < 'a > LintDiagnostic < 'a , () > ,) { if span . allows_unsafe () { return ; } cx . emit_span_lint (UNSAFE_CODE , span , decorate) ; } }
};
}
