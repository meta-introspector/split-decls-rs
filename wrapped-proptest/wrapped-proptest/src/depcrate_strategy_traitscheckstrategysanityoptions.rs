// Generated macro for CheckStrategySanityOptions (struct)
macro_rules! Depcrate_strategy_traitsCheckStrategySanityOptions {
() => {
// Module: crate::strategy::traits
// Provides: {"CheckStrategySanityOptions"}
// Dependencies: {}
# [doc = " Options passed to `check_strategy_sanity()`."] # [derive (Clone , Copy , Debug)] pub struct CheckStrategySanityOptions { # [doc = " If true (the default), require that `complicate()` return `true` at"] # [doc = " least once after any call to `simplify()` which itself returns once."] # [doc = ""] # [doc = " This property is not required by contract, but many strategies are"] # [doc = " designed in a way that this is expected to hold."] pub strict_complicate_after_simplify : bool , # [doc = " If true, cause local rejects to return an error instead of retrying."] # [doc = " Defaults to false. Useful for testing behaviors around error handling."] pub error_on_local_rejects : bool , # [allow (missing_docs)] # [doc (hidden)] pub _non_exhaustive : () , }
};
}
