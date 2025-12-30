// Generated macro for impl_40 (impl)
macro_rules! Depcrate_from_idimpl_40 {
() => {
// Module: crate::from_id
// Provides: {"impl_40"}
// Dependencies: {}
impl From < GenericDefId > for GenericDef { fn from (def : GenericDefId) -> Self { match def { GenericDefId :: FunctionId (it) => GenericDef :: Function (it . into ()) , GenericDefId :: AdtId (it) => GenericDef :: Adt (it . into ()) , GenericDefId :: TraitId (it) => GenericDef :: Trait (it . into ()) , GenericDefId :: TypeAliasId (it) => GenericDef :: TypeAlias (it . into ()) , GenericDefId :: ImplId (it) => GenericDef :: Impl (it . into ()) , GenericDefId :: ConstId (it) => GenericDef :: Const (it . into ()) , GenericDefId :: StaticId (it) => GenericDef :: Static (it . into ()) , } } }
};
}
