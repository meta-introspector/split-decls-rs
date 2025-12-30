// Generated macro for RunnablesConfig (struct)
macro_rules! Depcrate_configRunnablesConfig {
() => {
// Module: crate::config
// Provides: {"RunnablesConfig"}
// Dependencies: {}
# [doc = " Configuration for runnable items, such as `main` function or tests."] # [derive (Debug , Clone)] pub struct RunnablesConfig { # [doc = " Custom command to be executed instead of `cargo` for runnables."] pub override_cargo : Option < String > , # [doc = " Additional arguments for the `cargo`, e.g. `--release`."] pub cargo_extra_args : Vec < String > , # [doc = " Additional arguments for the binary being run, if it is a test or benchmark."] pub extra_test_binary_args : Vec < String > , }
};
}
