// Generated macro for ScalarValidatorFn (type)
macro_rules! Depcrate_registryScalarValidatorFn {
() => {
// Module: crate::registry
// Provides: {"ScalarValidatorFn"}
// Dependencies: {}
# [doc = " A validator for scalar"] pub type ScalarValidatorFn = Arc < dyn Fn (& Value) -> bool + Send + Sync > ;
};
}
