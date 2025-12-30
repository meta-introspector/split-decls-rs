// Generated macro for NoUndefinedVariables (struct)
macro_rules! Depcrate_validation_rules_no_undefined_variablesNoUndefinedVariables {
() => {
// Module: crate::validation::rules::no_undefined_variables
// Provides: {"NoUndefinedVariables"}
// Dependencies: {}
# [derive (Default)] pub struct NoUndefinedVariables < 'a > { defined_variables : HashMap < Option < & 'a str > , (Pos , HashSet < & 'a str >) > , used_variables : HashMap < Scope < 'a > , HashMap < & 'a str , Pos > > , current_scope : Option < Scope < 'a > > , spreads : HashMap < Scope < 'a > , Vec < & 'a str > > , }
};
}
