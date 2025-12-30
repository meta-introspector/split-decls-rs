// Generated macro for should_call_clone_as_function (function)
macro_rules! Depcrate_tyshould_call_clone_as_function {
() => {
// Module: crate::ty
// Provides: {"should_call_clone_as_function"}
// Dependencies: {}
# [doc = " Returns true if `ty` is a type on which calling `Clone` through a function instead of"] # [doc = " as a method, such as `Arc::clone()` is considered idiomatic."] # [doc = ""] # [doc = " Lints should avoid suggesting to replace instances of `ty::Clone()` by `.clone()` for objects"] # [doc = " of those types."] pub fn should_call_clone_as_function (cx : & LateContext < '_ > , ty : Ty < '_ >) -> bool { matches ! (ty . opt_diag_name (cx) , Some (sym :: Arc | sym :: ArcWeak | sym :: Rc | sym :: RcWeak)) }
};
}
