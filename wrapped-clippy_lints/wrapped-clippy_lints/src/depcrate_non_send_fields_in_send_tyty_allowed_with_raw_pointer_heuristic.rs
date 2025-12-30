// Generated macro for ty_allowed_with_raw_pointer_heuristic (function)
macro_rules! Depcrate_non_send_fields_in_send_tyty_allowed_with_raw_pointer_heuristic {
() => {
// Module: crate::non_send_fields_in_send_ty
// Provides: {"ty_allowed_with_raw_pointer_heuristic"}
// Dependencies: {}
# [doc = " Heuristic to allow cases like `Vec<*const u8>`"] fn ty_allowed_with_raw_pointer_heuristic < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx > , send_trait : DefId) -> bool { if implements_trait (cx , ty , send_trait , & []) || is_copy (cx , ty) { return true ; } match ty . kind () { ty :: Tuple (fields) => fields . iter () . all (| ty | ty_allowed_with_raw_pointer_heuristic (cx , ty , send_trait)) , ty :: Array (ty , _) | ty :: Slice (ty) => ty_allowed_with_raw_pointer_heuristic (cx , * ty , send_trait) , ty :: Adt (_ , args) => { if contains_pointer_like (cx , ty) { args . iter () . all (| generic_arg | match generic_arg . kind () { GenericArgKind :: Type (ty) => ty_allowed_with_raw_pointer_heuristic (cx , ty , send_trait) , GenericArgKind :: Lifetime (_) | GenericArgKind :: Const (_) => true , }) } else { false } } , ty :: RawPtr (_ , _) => true , _ => false , } }
};
}
