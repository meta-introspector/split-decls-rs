// Generated macro for is_async_fn (function)
macro_rules! Depcrateis_async_fn {
() => {
// Module: crate
// Provides: {"is_async_fn"}
// Dependencies: {}
# [doc = " Checks if the given function kind is an async function."] pub fn is_async_fn (kind : FnKind < '_ >) -> bool { match kind { FnKind :: ItemFn (_ , _ , header) => header . asyncness . is_async () , FnKind :: Method (_ , sig) => sig . header . asyncness . is_async () , FnKind :: Closure => false , } }
};
}
