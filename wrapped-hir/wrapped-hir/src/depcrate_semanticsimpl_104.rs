// Generated macro for impl_104 (impl)
macro_rules! Depcrate_semanticsimpl_104 {
() => {
// Module: crate::semantics
// Provides: {"impl_104"}
// Dependencies: {}
impl PathResolution { pub (crate) fn in_type_ns (& self) -> Option < TypeNs > { match self { PathResolution :: Def (ModuleDef :: Adt (adt)) => Some (TypeNs :: AdtId ((* adt) . into ())) , PathResolution :: Def (ModuleDef :: BuiltinType (builtin)) => { Some (TypeNs :: BuiltinType ((* builtin) . into ())) } PathResolution :: Def (ModuleDef :: Const (_) | ModuleDef :: Variant (_) | ModuleDef :: Macro (_) | ModuleDef :: Function (_) | ModuleDef :: Module (_) | ModuleDef :: Static (_) | ModuleDef :: Trait (_) | ModuleDef :: TraitAlias (_) ,) => None , PathResolution :: Def (ModuleDef :: TypeAlias (alias)) => { Some (TypeNs :: TypeAliasId ((* alias) . into ())) } PathResolution :: BuiltinAttr (_) | PathResolution :: ToolModule (_) | PathResolution :: Local (_) | PathResolution :: DeriveHelper (_) | PathResolution :: ConstParam (_) => None , PathResolution :: TypeParam (param) => Some (TypeNs :: GenericParam ((* param) . into ())) , PathResolution :: SelfType (impl_def) => Some (TypeNs :: SelfType ((* impl_def) . into ())) , } } }
};
}
