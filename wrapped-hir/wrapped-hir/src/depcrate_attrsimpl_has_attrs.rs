// Generated macro for impl_has_attrs (macro)
macro_rules! Depcrate_attrsimpl_has_attrs {
() => {
// Module: crate::attrs
// Provides: {"impl_has_attrs"}
// Dependencies: {}
macro_rules ! impl_has_attrs { ($ (($ def : ident , $ def_id : ident) ,) *) => { $ (impl HasAttrs for $ def { fn attrs (self , db : & dyn HirDatabase) -> AttrsWithOwner { let def = AttrDefId ::$ def_id (self . into ()) ; AttrsWithOwner :: new (db , def) } fn attr_id (self) -> AttrDefId { AttrDefId ::$ def_id (self . into ()) } }) * } ; }
};
}
