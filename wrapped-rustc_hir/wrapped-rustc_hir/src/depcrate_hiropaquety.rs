// Generated macro for OpaqueTy (struct)
macro_rules! Depcrate_hirOpaqueTy {
() => {
// Module: crate::hir
// Provides: {"OpaqueTy"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct OpaqueTy < 'hir > { # [stable_hasher (ignore)] pub hir_id : HirId , pub def_id : LocalDefId , pub bounds : GenericBounds < 'hir > , pub origin : OpaqueTyOrigin < LocalDefId > , pub span : Span , }
};
}
