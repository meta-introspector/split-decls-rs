// Generated macro for push_const_arg_has_type_predicates (function)
macro_rules! Depcrate_lowerpush_const_arg_has_type_predicates {
() => {
// Module: crate::lower
// Provides: {"push_const_arg_has_type_predicates"}
// Dependencies: {}
fn push_const_arg_has_type_predicates < 'db > (db : & 'db dyn HirDatabase , predicates : & mut Vec < Clause < 'db > > , generics : & Generics ,) { let interner = DbInterner :: new_with (db , None , None) ; let const_params_offset = generics . len_parent () + generics . len_lifetimes_self () ; for (param_index , (param_idx , param_data)) in generics . iter_self_type_or_consts () . enumerate () { if ! matches ! (param_data , TypeOrConstParamData :: ConstParamData (_)) { continue ; } let param_id = ConstParamId :: from_unchecked (TypeOrConstParamId { parent : generics . def () , local_id : param_idx , }) ; predicates . push (Clause (ClauseKind :: ConstArgHasType (Const :: new_param (interner , ParamConst { id : param_id , index : (param_index + const_params_offset) as u32 } ,) , db . const_param_ty_ns (param_id) ,) . upcast (interner) ,)) ; } }
};
}
