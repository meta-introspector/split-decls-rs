// Generated macro for impl_844 (impl)
macro_rules! Depcrate_validation_rules_no_unused_variablesimpl_844 {
() => {
// Module: crate::validation::rules::no_unused_variables
// Provides: {"impl_844"}
// Dependencies: {}
impl < 'a > NoUnusedVariables < 'a > { fn find_used_vars (& 'a self , from : & Scope < 'a > , defined : & HashSet < & 'a str > , used : & mut HashSet < & 'a str > , visited : & mut HashSet < Scope < 'a > > ,) { let mut to_visit = Vec :: new () ; if let Some (spreads) = self . find_used_vars_inner (from , defined , used , visited) { to_visit . push (spreads) ; } while let Some (spreads) = to_visit . pop () { for spread in spreads { if let Some (spreads) = self . find_used_vars_inner (& Scope :: Fragment (spread) , defined , used , visited) { to_visit . push (spreads) ; } } } } # [doc = " This function should be called only inside [`Self::find_used_vars()`],"] # [doc = " as it's a recursive function using heap instead of a stack. So, instead"] # [doc = " of the recursive call, we return a [`Vec`] that is visited inside"] # [doc = " [`Self::find_used_vars()`]."] fn find_used_vars_inner (& 'a self , from : & Scope < 'a > , defined : & HashSet < & 'a str > , used : & mut HashSet < & 'a str > , visited : & mut HashSet < Scope < 'a > > ,) -> Option < & 'a Vec < & 'a str > > { if visited . contains (from) { return None ; } visited . insert (from . clone ()) ; if let Some (used_vars) = self . used_variables . get (from) { for var in used_vars { if defined . contains (var) { used . insert (var) ; } } } self . spreads . get (from) } }
};
}
