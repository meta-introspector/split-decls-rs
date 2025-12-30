// Generated macro for impl_39 (impl)
macro_rules! Depcrate_from_idimpl_39 {
() => {
// Module: crate::from_id
// Provides: {"impl_39"}
// Dependencies: {}
impl From < GenericDef > for GenericDefId { fn from (def : GenericDef) -> Self { match def { GenericDef :: Function (it) => GenericDefId :: FunctionId (it . id) , GenericDef :: Adt (it) => GenericDefId :: AdtId (it . into ()) , GenericDef :: Trait (it) => GenericDefId :: TraitId (it . id) , GenericDef :: TypeAlias (it) => GenericDefId :: TypeAliasId (it . id) , GenericDef :: Impl (it) => GenericDefId :: ImplId (it . id) , GenericDef :: Const (it) => GenericDefId :: ConstId (it . id) , GenericDef :: Static (it) => GenericDefId :: StaticId (it . id) , } } }
};
}
