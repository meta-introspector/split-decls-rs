// Generated macro for resolve_hir_value_path (function)
macro_rules! Depcrate_source_analyzerresolve_hir_value_path {
() => {
// Module: crate::source_analyzer
// Provides: {"resolve_hir_value_path"}
// Dependencies: {}
fn resolve_hir_value_path (db : & dyn HirDatabase , resolver : & Resolver < '_ > , body_owner : Option < DefWithBodyId > , path : & Path , hygiene : HygieneId ,) -> Option < PathResolution > { resolver . resolve_path_in_value_ns_fully (db , path , hygiene) . and_then (| val | { let res = match val { ValueNs :: LocalBinding (binding_id) => { let var = Local { parent : body_owner ? , binding_id } ; PathResolution :: Local (var) } ValueNs :: FunctionId (it) => PathResolution :: Def (Function :: from (it) . into ()) , ValueNs :: ConstId (it) => PathResolution :: Def (Const :: from (it) . into ()) , ValueNs :: StaticId (it) => PathResolution :: Def (Static :: from (it) . into ()) , ValueNs :: StructId (it) => PathResolution :: Def (Struct :: from (it) . into ()) , ValueNs :: EnumVariantId (it) => PathResolution :: Def (Variant :: from (it) . into ()) , ValueNs :: ImplSelf (impl_id) => PathResolution :: SelfType (impl_id . into ()) , ValueNs :: GenericParam (id) => PathResolution :: ConstParam (id . into ()) , } ; Some (res) }) }
};
}
