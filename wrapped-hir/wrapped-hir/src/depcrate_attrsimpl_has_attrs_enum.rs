// Generated macro for impl_has_attrs_enum (macro)
macro_rules! Depcrate_attrsimpl_has_attrs_enum {
() => {
// Module: crate::attrs
// Provides: {"impl_has_attrs_enum"}
// Dependencies: {}
macro_rules ! impl_has_attrs_enum { ($ ($ variant : ident) ,* for $ enum : ident) => { $ (impl HasAttrs for $ variant { fn attrs (self , db : & dyn HirDatabase) -> AttrsWithOwner { $ enum ::$ variant (self) . attrs (db) } fn attr_id (self) -> AttrDefId { $ enum ::$ variant (self) . attr_id () } }) * } ; }
};
}
