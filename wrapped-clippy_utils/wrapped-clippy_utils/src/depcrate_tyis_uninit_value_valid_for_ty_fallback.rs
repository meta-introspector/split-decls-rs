// Generated macro for is_uninit_value_valid_for_ty_fallback (function)
macro_rules! Depcrate_tyis_uninit_value_valid_for_ty_fallback {
() => {
// Module: crate::ty
// Provides: {"is_uninit_value_valid_for_ty_fallback"}
// Dependencies: {}
# [doc = " A fallback for polymorphic types, which are not supported by `check_validity_requirement`."] fn is_uninit_value_valid_for_ty_fallback < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { match * ty . kind () { ty :: Array (component , _) => is_uninit_value_valid_for_ty (cx , component) , ty :: Tuple (types) => types . iter () . all (| ty | is_uninit_value_valid_for_ty (cx , ty)) , ty :: Adt (adt , _) if adt . is_union () => true , ty :: Adt (adt , args) if adt . is_struct () => adt . all_fields () . all (| field | is_uninit_value_valid_for_ty (cx , field . ty (cx . tcx , args))) , _ => false , } }
};
}
