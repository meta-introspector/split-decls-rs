// Generated macro for ref_type_with (function)
macro_rules! Depcrate_item_implref_type_with {
() => {
// Module: crate::item_impl
// Provides: {"ref_type_with"}
// Dependencies: {}
fn ref_type_with (ty : & Type , is_ref : bool) -> Type { if is_ref { ref_type (ty) } else { ty . clone () } }
};
}
