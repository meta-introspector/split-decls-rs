// Generated macro for has_mutable_arg (function)
macro_rules! Depcrate_functions_must_usehas_mutable_arg {
() => {
// Module: crate::functions::must_use
// Provides: {"has_mutable_arg"}
// Dependencies: {}
fn has_mutable_arg (cx : & LateContext < '_ > , body : & hir :: Body < '_ >) -> bool { let mut tys = DefIdSet :: default () ; body . params . iter () . any (| param | is_mutable_pat (cx , param . pat , & mut tys)) }
};
}
