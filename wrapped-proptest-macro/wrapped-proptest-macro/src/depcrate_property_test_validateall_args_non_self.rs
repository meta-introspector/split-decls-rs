// Generated macro for all_args_non_self (function)
macro_rules! Depcrate_property_test_validateall_args_non_self {
() => {
// Module: crate::property_test::validate
// Provides: {"all_args_non_self"}
// Dependencies: {}
fn all_args_non_self (f : & mut ItemFn) -> Result < () , TokenStream > { let first_self_arg = f . sig . inputs . iter () . find (| arg | matches ! (arg , FnArg :: Receiver (_))) ; match first_self_arg { None => Ok (()) , Some (arg) => err (arg , "`self` parameters are forbidden") , } }
};
}
