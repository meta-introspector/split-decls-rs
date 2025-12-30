// Generated macro for impl_209 (impl)
macro_rules! Depcrate_contextimpl_209 {
() => {
// Module: crate::context
// Provides: {"impl_209"}
// Dependencies: {}
impl < 'tcx > LintContext for LateContext < 'tcx > { # [doc = " Gets the overall compiler `Session` object."] fn sess (& self) -> & Session { self . tcx . sess } # [rustc_lint_diagnostics] fn opt_span_lint < S : Into < MultiSpan > > (& self , lint : & 'static Lint , span : Option < S > , decorate : impl for < 'a , 'b > FnOnce (& 'b mut Diag < 'a , () >) ,) { let hir_id = self . last_node_with_lint_attrs ; match span { Some (s) => self . tcx . node_span_lint (lint , hir_id , s , decorate) , None => self . tcx . node_lint (lint , hir_id , decorate) , } } fn get_lint_level (& self , lint : & 'static Lint) -> LevelAndSource { self . tcx . lint_level_at_node (lint , self . last_node_with_lint_attrs) } }
};
}
