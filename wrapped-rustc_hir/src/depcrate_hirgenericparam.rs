// Generated macro for GenericParam (struct)
macro_rules! Depcrate_hirGenericParam {
() => {
// Module: crate::hir
// Provides: {"GenericParam"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct GenericParam < 'hir > { # [stable_hasher (ignore)] pub hir_id : HirId , pub def_id : LocalDefId , pub name : ParamName , pub span : Span , pub pure_wrt_drop : bool , pub kind : GenericParamKind < 'hir > , pub colon_span : Option < Span > , pub source : GenericParamSource , }
};
}
