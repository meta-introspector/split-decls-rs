// Generated macro for strip_args (function)
macro_rules! Depcrate_property_test_utilsstrip_args {
() => {
// Module: crate::property_test::utils
// Provides: {"strip_args"}
// Dependencies: {}
# [doc = " Convert a function to a zero-arg function, and return the args"] # [doc = ""] # [doc = " Panics on any invalid function"] pub fn strip_args (mut f : ItemFn) -> (ItemFn , Vec < Argument >) { let args = std :: mem :: take (& mut f . sig . inputs) ; let args = args . into_iter () . map (| arg | match arg { FnArg :: Typed (arg) => strip_strategy (arg) , FnArg :: Receiver (_) => panic ! ("receivers aren't allowed - should be filtered by `validate`") , }) . collect () ; (f , args) }
};
}
