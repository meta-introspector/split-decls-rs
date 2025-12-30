// Generated macro for to_ref_elem (function)
macro_rules! Depcrate_item_implto_ref_elem {
() => {
// Module: crate::item_impl
// Provides: {"to_ref_elem"}
// Dependencies: {}
fn to_ref_elem (ty : & Type) -> (Type , bool) { if let Type :: Reference (tr) = ty { if tr . lifetime . is_none () && tr . mutability . is_none () { return (tr . elem . as_ref () . clone () , true) ; } } (ty . clone () , false) }
};
}
