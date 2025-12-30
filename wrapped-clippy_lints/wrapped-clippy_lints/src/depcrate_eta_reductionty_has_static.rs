// Generated macro for ty_has_static (function)
macro_rules! Depcrate_eta_reductionty_has_static {
() => {
// Module: crate::eta_reduction
// Provides: {"ty_has_static"}
// Dependencies: {}
fn ty_has_static (ty : Ty < '_ >) -> bool { ty . walk () . any (| arg | matches ! (arg . kind () , GenericArgKind :: Lifetime (re) if re . is_static ())) }
};
}
