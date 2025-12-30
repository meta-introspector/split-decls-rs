// Generated macro for Options (struct)
macro_rules! Depcrate_property_test_optionsOptions {
() => {
// Module: crate::property_test::options
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options parsed from the attribute itself (e.g. the config from `#[property_test(config = ...)]`)"] # [derive (Default)] pub (super) struct Options { # [doc = " Collect compiler errors and emit them later, since errors here are largely recoverable"] pub errors : Vec < TokenStream > , pub config : Option < Expr > , }
};
}
