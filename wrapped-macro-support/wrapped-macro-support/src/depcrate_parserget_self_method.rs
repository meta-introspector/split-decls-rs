// Generated macro for get_self_method (function)
macro_rules! Depcrate_parserget_self_method {
() => {
// Module: crate::parser
// Provides: {"get_self_method"}
// Dependencies: {}
# [doc = " Returns whether `self` is passed by reference or by value."] fn get_self_method (r : syn :: Receiver) -> ast :: MethodSelf { match & * r . ty { syn :: Type :: Reference (ty) => { if ty . mutability . is_some () { ast :: MethodSelf :: RefMutable } else { ast :: MethodSelf :: RefShared } } _ => ast :: MethodSelf :: ByValue , } }
};
}
