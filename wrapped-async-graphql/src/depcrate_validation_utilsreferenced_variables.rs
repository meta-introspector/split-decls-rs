// Generated macro for referenced_variables (function)
macro_rules! Depcrate_validation_utilsreferenced_variables {
() => {
// Module: crate::validation::utils
// Provides: {"referenced_variables"}
// Dependencies: {}
pub fn referenced_variables (value : & Value) -> Vec < & str > { let mut vars = Vec :: new () ; referenced_variables_to_vec (value , & mut vars) ; vars }
};
}
