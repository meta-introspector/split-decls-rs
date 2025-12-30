// Generated macro for rust_lld (function)
macro_rules! Depcrate_core_config_testsrust_lld {
() => {
// Module: crate::core::config::tests
// Provides: {"rust_lld"}
// Dependencies: {}
# [test] fn rust_lld () { assert ! (matches ! (parse ("") . lld_mode , LldMode :: Unused)) ; assert ! (matches ! (parse ("rust.use-lld = \"self-contained\"") . lld_mode , LldMode :: SelfContained)) ; assert ! (matches ! (parse ("rust.use-lld = \"external\"") . lld_mode , LldMode :: External)) ; assert ! (matches ! (parse ("rust.use-lld = true") . lld_mode , LldMode :: External)) ; assert ! (matches ! (parse ("rust.use-lld = false") . lld_mode , LldMode :: Unused)) ; }
};
}
