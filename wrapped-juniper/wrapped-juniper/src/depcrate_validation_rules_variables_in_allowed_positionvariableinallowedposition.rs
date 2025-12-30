// Generated macro for VariableInAllowedPosition (struct)
macro_rules! Depcrate_validation_rules_variables_in_allowed_positionVariableInAllowedPosition {
() => {
// Module: crate::validation::rules::variables_in_allowed_position
// Provides: {"VariableInAllowedPosition"}
// Dependencies: {}
pub struct VariableInAllowedPosition < 'a , S : fmt :: Debug + 'a > { spreads : HashMap < Scope < 'a > , HashSet < & 'a str > > , variable_usages : HashMap < Scope < 'a > , Vec < (SpannedInput < 'a , String > , BorrowedType < 'a >) > > , # [expect (clippy :: type_complexity , reason = "readable enough")] variable_defs : HashMap < Scope < 'a > , Vec < & 'a (Spanning < & 'a str > , VariableDefinition < 'a , S >) > > , current_scope : Option < Scope < 'a > > , }
};
}
