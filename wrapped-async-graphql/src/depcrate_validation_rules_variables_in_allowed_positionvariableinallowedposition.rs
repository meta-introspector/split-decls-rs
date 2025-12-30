// Generated macro for VariableInAllowedPosition (struct)
macro_rules! Depcrate_validation_rules_variables_in_allowed_positionVariableInAllowedPosition {
() => {
// Module: crate::validation::rules::variables_in_allowed_position
// Provides: {"VariableInAllowedPosition"}
// Dependencies: {}
# [derive (Default)] pub struct VariableInAllowedPosition < 'a > { spreads : HashMap < Scope < 'a > , HashSet < & 'a str > > , variable_usages : HashMap < Scope < 'a > , Vec < (& 'a str , Pos , MetaTypeName < 'a >) > > , variable_defs : HashMap < Scope < 'a > , Vec < & 'a Positioned < VariableDefinition > > > , current_scope : Option < Scope < 'a > > , }
};
}
