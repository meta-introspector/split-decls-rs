// Generated macro for contains_pointer_like (function)
macro_rules! Depcrate_non_send_fields_in_send_tycontains_pointer_like {
() => {
// Module: crate::non_send_fields_in_send_ty
// Provides: {"contains_pointer_like"}
// Dependencies: {}
# [doc = " Checks if the type contains any pointer-like types in args (including nested ones)"] fn contains_pointer_like < 'tcx > (cx : & LateContext < 'tcx > , target_ty : Ty < 'tcx >) -> bool { for ty_node in target_ty . walk () { if let GenericArgKind :: Type (inner_ty) = ty_node . kind () { match inner_ty . kind () { ty :: RawPtr (_ , _) => { return true ; } , ty :: Adt (adt_def , _) => { if cx . tcx . is_diagnostic_item (sym :: NonNull , adt_def . did ()) { return true ; } } , _ => () , } } } false }
};
}
