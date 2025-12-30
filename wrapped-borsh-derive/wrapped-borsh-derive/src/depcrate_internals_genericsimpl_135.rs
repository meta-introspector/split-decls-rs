// Generated macro for impl_135 (impl)
macro_rules! Depcrate_internals_genericsimpl_135 {
() => {
// Module: crate::internals::generics
// Provides: {"impl_135"}
// Dependencies: {}
# [cfg (feature = "schema")] impl FindTyParams { pub fn from_params < 'a > (params : impl Iterator < Item = & 'a Ident >) -> Self { let all_type_params_ordered : Vec < Ident > = params . cloned () . collect () ; let all_type_params = all_type_params_ordered . clone () . into_iter () . collect () ; FindTyParams { all_type_params , all_type_params_ordered , relevant_type_params : HashSet :: new () , associated_type_params_usage : HashMap :: new () , } } pub fn process_for_params (self) -> Vec < Ident > { let relevant_type_params = self . relevant_type_params ; let associated_type_params_usage = self . associated_type_params_usage ; let mut params : Vec < Ident > = vec ! [] ; let mut params_set : HashSet < Ident > = HashSet :: new () ; self . all_type_params_ordered . iter () . for_each (| param | { if relevant_type_params . contains (param) && ! params_set . contains (param) { params . push (param . clone ()) ; params_set . insert (param . clone ()) ; } if associated_type_params_usage . contains_key (param) && ! params_set . contains (param) { params . push (param . clone ()) ; params_set . insert (param . clone ()) ; } }) ; params } pub fn at_least_one_hit (& self) -> bool { ! self . relevant_type_params . is_empty () || ! self . associated_type_params_usage . is_empty () } }
};
}
