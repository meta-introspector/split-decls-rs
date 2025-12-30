// Generated macro for NoUndefinedVariables (struct)
macro_rules! Depcrate_validation_rules_no_undefined_variablesNoUndefinedVariables {
() => {
// Module: crate::validation::rules::no_undefined_variables
// Provides: {"NoUndefinedVariables"}
// Dependencies: {}
pub struct NoUndefinedVariables < 'a > { defined_variables : HashMap < Option < & 'a str > , (SourcePosition , HashSet < & 'a str >) > , used_variables : HashMap < Scope < 'a > , Vec < BorrowedSpanning < 'a , str > > > , current_scope : Option < Scope < 'a > > , spreads : HashMap < Scope < 'a > , Vec < & 'a str > > , }
};
}
