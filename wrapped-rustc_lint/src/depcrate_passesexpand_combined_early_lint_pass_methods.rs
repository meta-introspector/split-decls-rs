// Generated macro for expand_combined_early_lint_pass_methods (macro)
macro_rules! Depcrate_passesexpand_combined_early_lint_pass_methods {
() => {
// Module: crate::passes
// Provides: {"expand_combined_early_lint_pass_methods"}
// Dependencies: {}
# [macro_export] macro_rules ! expand_combined_early_lint_pass_methods { ($ passes : tt , [$ ($ (# [$ attr : meta]) * fn $ name : ident ($ ($ param : ident : $ arg : ty) ,*) ;) *]) => ($ (fn $ name (& mut self , context : &$ crate :: EarlyContext <'_ >, $ ($ param : $ arg) ,*) { $ crate :: expand_combined_early_lint_pass_method ! ($ passes , self , $ name , (context , $ ($ param) ,*)) ; }) *) }
};
}
