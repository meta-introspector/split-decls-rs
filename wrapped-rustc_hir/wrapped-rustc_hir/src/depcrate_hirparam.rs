// Generated macro for Param (struct)
macro_rules! Depcrate_hirParam {
() => {
// Module: crate::hir
// Provides: {"Param"}
// Dependencies: {}
# [doc = " Represents a parameter in a function header."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct Param < 'hir > { # [stable_hasher (ignore)] pub hir_id : HirId , pub pat : & 'hir Pat < 'hir > , pub ty_span : Span , pub span : Span , }
};
}
