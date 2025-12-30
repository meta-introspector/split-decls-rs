// Generated macro for factory (function)
macro_rules! Depcrate_validation_rules_no_undefined_variablesfactory {
() => {
// Module: crate::validation::rules::no_undefined_variables
// Provides: {"factory"}
// Dependencies: {}
pub fn factory < 'a > () -> NoUndefinedVariables < 'a > { NoUndefinedVariables { defined_variables : HashMap :: new () , used_variables : HashMap :: new () , current_scope : None , spreads : HashMap :: new () , } }
};
}
