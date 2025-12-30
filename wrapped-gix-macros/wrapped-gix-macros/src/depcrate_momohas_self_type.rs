// Generated macro for has_self_type (function)
macro_rules! Depcrate_momohas_self_type {
() => {
// Module: crate::momo
// Provides: {"has_self_type"}
// Dependencies: {}
fn has_self_type (input : & FnArg) -> bool { match input { FnArg :: Receiver (_) => true , FnArg :: Typed (PatType { ty , .. }) => contains_self_type (ty) , } }
};
}
