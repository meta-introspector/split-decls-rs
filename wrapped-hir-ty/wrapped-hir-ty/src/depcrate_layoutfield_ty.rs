// Generated macro for field_ty (function)
macro_rules! Depcrate_layoutfield_ty {
() => {
// Module: crate::layout
// Provides: {"field_ty"}
// Dependencies: {}
fn field_ty < 'a > (db : & 'a dyn HirDatabase , def : hir_def :: VariantId , fd : LocalFieldId , args : & GenericArgs < 'a > ,) -> Ty < 'a > { db . field_types (def) [fd] . instantiate (DbInterner :: new_with (db , None , None) , args) }
};
}
