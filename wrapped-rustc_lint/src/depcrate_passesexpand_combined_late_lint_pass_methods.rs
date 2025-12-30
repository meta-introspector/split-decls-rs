// Generated macro for expand_combined_late_lint_pass_methods (macro)
macro_rules! Depcrate_passesexpand_combined_late_lint_pass_methods {
() => {
// Module: crate::passes
// Provides: {"expand_combined_late_lint_pass_methods"}
// Dependencies: {}
# [macro_export] macro_rules ! expand_combined_late_lint_pass_methods { ($ passes : tt , [$ ($ (# [$ attr : meta]) * fn $ name : ident ($ ($ param : ident : $ arg : ty) ,*) ;) *]) => ($ (fn $ name (& mut self , context : &$ crate :: LateContext <'tcx >, $ ($ param : $ arg) ,*) { $ crate :: expand_combined_late_lint_pass_method ! ($ passes , self , $ name , (context , $ ($ param) ,*)) ; }) *) }
};
}
