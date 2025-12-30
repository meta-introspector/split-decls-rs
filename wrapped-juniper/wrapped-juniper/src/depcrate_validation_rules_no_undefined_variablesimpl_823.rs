// Generated macro for impl_823 (impl)
macro_rules! Depcrate_validation_rules_no_undefined_variablesimpl_823 {
() => {
// Module: crate::validation::rules::no_undefined_variables
// Provides: {"impl_823"}
// Dependencies: {}
impl < 'a > NoUndefinedVariables < 'a > { fn find_undef_vars (& 'a self , scope : & Scope < 'a > , defined : & HashSet < & 'a str > , unused : & mut Vec < BorrowedSpanning < 'a , str > > , visited : & mut HashSet < Scope < 'a > > ,) { let mut to_visit = Vec :: new () ; if let Some (spreads) = self . find_undef_vars_inner (scope , defined , unused , visited) { to_visit . push (spreads) ; } while let Some (spreads) = to_visit . pop () { for spread in spreads { if let Some (spreads) = self . find_undef_vars_inner (& Scope :: Fragment (spread) , defined , unused , visited) { to_visit . push (spreads) ; } } } } # [doc = " This function should be called only inside [`Self::find_undef_vars()`],"] # [doc = " as it's a recursive function using heap instead of a stack. So, instead"] # [doc = " of the recursive call, we return a [`Vec`] that is visited inside"] # [doc = " [`Self::find_undef_vars()`]."] fn find_undef_vars_inner (& 'a self , scope : & Scope < 'a > , defined : & HashSet < & 'a str > , unused : & mut Vec < BorrowedSpanning < 'a , str > > , visited : & mut HashSet < Scope < 'a > > ,) -> Option < & 'a Vec < & 'a str > > { if visited . contains (scope) { return None ; } visited . insert (scope . clone ()) ; if let Some (used_vars) = self . used_variables . get (scope) { for var in used_vars { if ! defined . contains (& var . item) { unused . push (* var) ; } } } self . spreads . get (scope) } }
};
}
