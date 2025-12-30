// Generated macro for impl_298 (impl)
macro_rules! Depcrate_validation_rules_no_undefined_variablesimpl_298 {
() => {
// Module: crate::validation::rules::no_undefined_variables
// Provides: {"impl_298"}
// Dependencies: {}
impl < 'a > NoUndefinedVariables < 'a > { fn find_undef_vars (& 'a self , scope : & Scope < 'a > , defined : & HashSet < & 'a str > , undef : & mut Vec < (& 'a str , Pos) > , visited : & mut HashSet < Scope < 'a > > ,) { if visited . contains (scope) { return ; } visited . insert (* scope) ; if let Some (used_vars) = self . used_variables . get (scope) { for (var , pos) in used_vars { if ! defined . contains (var) { undef . push ((* var , * pos)) ; } } } if let Some (spreads) = self . spreads . get (scope) { for spread in spreads { self . find_undef_vars (& Scope :: Fragment (spread) , defined , undef , visited) ; } } } }
};
}
