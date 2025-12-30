// Generated macro for has_drop (function)
macro_rules! Depcrate_tyhas_drop {
() => {
// Module: crate::ty
// Provides: {"has_drop"}
// Dependencies: {}
# [doc = " Checks whether this type implements `Drop`."] pub fn has_drop < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { match ty . ty_adt_def () { Some (def) => def . has_dtor (cx . tcx) , None => false , } }
};
}
