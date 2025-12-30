// Generated macro for lint_callback (macro)
macro_rules! Depcrate_earlylint_callback {
() => {
// Module: crate::early
// Provides: {"lint_callback"}
// Dependencies: {}
macro_rules ! lint_callback { ($ cx : expr , $ f : ident , $ ($ args : expr) ,*) => ({ $ cx . pass .$ f (&$ cx . context , $ ($ args) ,*) ; }) }
};
}
