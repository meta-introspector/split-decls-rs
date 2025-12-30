// Generated macro for impl_432 (impl)
macro_rules! Depcrate_resolverimpl_432 {
() => {
// Module: crate::resolver
// Provides: {"impl_432"}
// Dependencies: {}
impl < 'db > ModuleItemMap < 'db > { fn resolve_path_in_value_ns (& self , db : & 'db dyn DefDatabase , path : & ModPath ,) -> Option < (ResolveValueResult , ResolvePathResultPrefixInfo) > { let (module_def , unresolved_idx , prefix_info) = self . def_map . resolve_path_locally (self . local_def_map , db , self . module_id , path , BuiltinShadowMode :: Other ,) ; match unresolved_idx { None => { let (value , import) = to_value_ns (module_def) ? ; Some ((ResolveValueResult :: ValueNs (value , import) , prefix_info)) } Some (unresolved_idx) => { let def = module_def . take_types_full () ? ; let ty = match def . def { ModuleDefId :: AdtId (it) => TypeNs :: AdtId (it) , ModuleDefId :: TraitId (it) => TypeNs :: TraitId (it) , ModuleDefId :: TypeAliasId (it) => TypeNs :: TypeAliasId (it) , ModuleDefId :: BuiltinType (it) => TypeNs :: BuiltinType (it) , ModuleDefId :: ModuleId (_) | ModuleDefId :: FunctionId (_) | ModuleDefId :: EnumVariantId (_) | ModuleDefId :: ConstId (_) | ModuleDefId :: MacroId (_) | ModuleDefId :: StaticId (_) => return None , } ; Some ((ResolveValueResult :: Partial (ty , unresolved_idx , def . import) , prefix_info)) } } } fn resolve_path_in_type_ns (& self , db : & dyn DefDatabase , path : & ModPath ,) -> Option < (TypeNs , Option < usize > , Option < ImportOrExternCrate > , ResolvePathResultPrefixInfo) > { let (module_def , idx , prefix_info) = self . def_map . resolve_path_locally (self . local_def_map , db , self . module_id , path , BuiltinShadowMode :: Other ,) ; let (res , import) = to_type_ns (module_def) ? ; Some ((res , idx , import , prefix_info)) } }
};
}
