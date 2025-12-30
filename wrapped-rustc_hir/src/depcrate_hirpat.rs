// Generated macro for Pat (struct)
macro_rules! Depcrate_hirPat {
() => {
// Module: crate::hir
// Provides: {"Pat"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct Pat < 'hir > { # [stable_hasher (ignore)] pub hir_id : HirId , pub kind : PatKind < 'hir > , pub span : Span , # [doc = " Whether to use default binding modes."] # [doc = " At present, this is false only for destructuring assignment."] pub default_binding_modes : bool , }
};
}
