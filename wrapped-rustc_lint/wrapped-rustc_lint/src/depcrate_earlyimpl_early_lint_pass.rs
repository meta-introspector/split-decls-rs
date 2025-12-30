// Generated macro for impl_early_lint_pass (macro)
macro_rules! Depcrate_earlyimpl_early_lint_pass {
() => {
// Module: crate::early
// Provides: {"impl_early_lint_pass"}
// Dependencies: {}
macro_rules ! impl_early_lint_pass { ([] , [$ ($ (# [$ attr : meta]) * fn $ f : ident ($ ($ param : ident : $ arg : ty) ,*) ;) *]) => (impl EarlyLintPass for RuntimeCombinedEarlyLintPass <'_ > { $ (fn $ f (& mut self , context : & EarlyContext <'_ >, $ ($ param : $ arg) ,*) { for pass in self . passes . iter_mut () { pass .$ f (context , $ ($ param) ,*) ; } }) * }) }
};
}
