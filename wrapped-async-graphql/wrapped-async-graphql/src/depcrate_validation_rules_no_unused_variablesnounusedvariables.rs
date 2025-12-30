// Generated macro for NoUnusedVariables (struct)
macro_rules! Depcrate_validation_rules_no_unused_variablesNoUnusedVariables {
() => {
// Module: crate::validation::rules::no_unused_variables
// Provides: {"NoUnusedVariables"}
// Dependencies: {}
# [derive (Default)] pub struct NoUnusedVariables < 'a > { defined_variables : HashMap < Option < & 'a str > , HashSet < (& 'a str , Pos) > > , used_variables : HashMap < Scope < 'a > , Vec < & 'a str > > , current_scope : Option < Scope < 'a > > , spreads : HashMap < Scope < 'a > , Vec < & 'a str > > , }
};
}
