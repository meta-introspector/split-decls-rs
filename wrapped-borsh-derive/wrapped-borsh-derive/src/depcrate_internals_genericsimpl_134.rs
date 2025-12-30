// Generated macro for impl_134 (impl)
macro_rules! Depcrate_internals_genericsimpl_134 {
() => {
// Module: crate::internals::generics
// Provides: {"impl_134"}
// Dependencies: {}
impl FindTyParams { pub fn new (generics : & Generics) -> Self { let all_type_params = generics . type_params () . map (| param | param . ident . clone ()) . collect () ; let all_type_params_ordered = generics . type_params () . map (| param | param . ident . clone ()) . collect () ; FindTyParams { all_type_params , all_type_params_ordered , relevant_type_params : HashSet :: new () , associated_type_params_usage : HashMap :: new () , } } pub fn process_for_bounds (self) -> Vec < Type > { let relevant_type_params = self . relevant_type_params ; let associated_type_params_usage = self . associated_type_params_usage ; let mut new_predicates : Vec < Type > = vec ! [] ; let mut new_predicates_set : HashSet < String > = HashSet :: new () ; self . all_type_params_ordered . iter () . for_each (| param | { if relevant_type_params . contains (param) { let ty = Type :: Path (TypePath { qself : None , path : param . clone () . into () , }) ; let ty_str_repr = ty . to_token_stream () . to_string () ; if ! new_predicates_set . contains (& ty_str_repr) { new_predicates . push (ty) ; new_predicates_set . insert (ty_str_repr) ; } } if let Some (vec_type) = associated_type_params_usage . get (param) { for type_ in vec_type { let ty_str_repr = type_ . to_token_stream () . to_string () ; if ! new_predicates_set . contains (& ty_str_repr) { new_predicates . push (type_ . clone ()) ; new_predicates_set . insert (ty_str_repr) ; } } } }) ; new_predicates } }
};
}
