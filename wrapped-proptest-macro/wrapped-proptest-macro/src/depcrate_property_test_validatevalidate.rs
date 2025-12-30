// Generated macro for validate (function)
macro_rules! Depcrate_property_test_validatevalidate {
() => {
// Module: crate::property_test::validate
// Provides: {"validate"}
// Dependencies: {}
# [doc = " Validate an `ItemFn` for some basic sanity checks"] # [doc = ""] # [doc = " Many checks are deferred to rustc (e.g. rustc already errors if you make a test function"] # [doc = " unsafe, so we just transparently pass unsafe through to the generated function and let rustc"] # [doc = " emit the error)"] pub (super) fn validate (f : & mut ItemFn) -> Result < () , TokenStream > { all_args_non_self (f) ? ; validate_parameter_attrs (f) ? ; Ok (()) }
};
}
