// Generated macro for impl_110 (impl)
macro_rules! Depcrate_item_scopeimpl_110 {
() => {
// Module: crate::item_scope
// Provides: {"impl_110"}
// Dependencies: {}
impl PerNs { pub (crate) fn from_def (def : ModuleDefId , v : Visibility , has_constructor : bool , import : Option < ImportOrExternCrate > ,) -> PerNs { match def { ModuleDefId :: ModuleId (_) => PerNs :: types (def , v , import) , ModuleDefId :: FunctionId (_) => { PerNs :: values (def , v , import . and_then (ImportOrExternCrate :: import_or_glob)) } ModuleDefId :: AdtId (adt) => match adt { AdtId :: UnionId (_) => PerNs :: types (def , v , import) , AdtId :: EnumId (_) => PerNs :: types (def , v , import) , AdtId :: StructId (_) => { if has_constructor { PerNs :: both (def , def , v , import) } else { PerNs :: types (def , v , import) } } } , ModuleDefId :: EnumVariantId (_) => PerNs :: both (def , def , v , import) , ModuleDefId :: ConstId (_) | ModuleDefId :: StaticId (_) => { PerNs :: values (def , v , import . and_then (ImportOrExternCrate :: import_or_glob)) } ModuleDefId :: TraitId (_) => PerNs :: types (def , v , import) , ModuleDefId :: TraitAliasId (_) => PerNs :: types (def , v , import) , ModuleDefId :: TypeAliasId (_) => PerNs :: types (def , v , import) , ModuleDefId :: BuiltinType (_) => PerNs :: types (def , v , import) , ModuleDefId :: MacroId (mac) => PerNs :: macros (mac , v , import) , } } }
};
}
