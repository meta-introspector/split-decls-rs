// Generated macro for contains_adt_constructor (function)
macro_rules! Depcrate_tycontains_adt_constructor {
() => {
// Module: crate::ty
// Provides: {"contains_adt_constructor"}
// Dependencies: {}
# [doc = " Walks into `ty` and returns `true` if any inner type is an instance of the given adt"] # [doc = " constructor."] pub fn contains_adt_constructor < 'tcx > (ty : Ty < 'tcx > , adt : AdtDef < 'tcx >) -> bool { ty . walk () . any (| inner | match inner . kind () { GenericArgKind :: Type (inner_ty) => inner_ty . ty_adt_def () == Some (adt) , GenericArgKind :: Lifetime (_) | GenericArgKind :: Const (_) => false , }) }
};
}
