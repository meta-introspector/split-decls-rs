// Generated macro for is_closure_like (function)
macro_rules! Depcrate_diagnostics_mutability_errorsis_closure_like {
() => {
// Module: crate::diagnostics::mutability_errors
// Provides: {"is_closure_like"}
// Dependencies: {}
# [doc = " If the type is a `Coroutine`, `Closure`, or `CoroutineClosure`"] fn is_closure_like (ty : Ty < '_ >) -> bool { ty . is_closure () || ty . is_coroutine () || ty . is_coroutine_closure () }
};
}
