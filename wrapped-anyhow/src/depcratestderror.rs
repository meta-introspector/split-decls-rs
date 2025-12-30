// Generated macro for StdError (trait)
macro_rules! DepcrateStdError {
() => {
// Module: crate
// Provides: {"StdError"}
// Dependencies: {}
# [cfg (all (not (feature = "std") , anyhow_no_core_error))] trait StdError : Debug + Display { fn source (& self) -> Option < & (dyn StdError + 'static) > { None } }
};
}
