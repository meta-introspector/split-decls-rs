// Generated macro for impl_14 (impl)
macro_rules! Depcrate_attrsimpl_14 {
() => {
// Module: crate::attrs
// Provides: {"impl_14"}
// Dependencies: {}
impl HasAttrs for crate :: Crate { fn attrs (self , db : & dyn HirDatabase) -> AttrsWithOwner { let def = AttrDefId :: ModuleId (self . root_module () . id) ; AttrsWithOwner :: new (db , def) } fn attr_id (self) -> AttrDefId { AttrDefId :: ModuleId (self . root_module () . id) } }
};
}
