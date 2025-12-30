// Generated macro for DeclOrigin (enum)
macro_rules! Depcrate_gather_localsDeclOrigin {
() => {
// Module: crate::gather_locals
// Provides: {"DeclOrigin"}
// Dependencies: {}
# [doc = " Provides context for checking patterns in declarations. More specifically this"] # [doc = " allows us to infer array types if the pattern is irrefutable and allows us to infer"] # [doc = " the size of the array. See issue #76342."] # [derive (Debug , Copy , Clone)] pub (super) enum DeclOrigin < 'a > { LetExpr , LocalDecl { els : Option < & 'a hir :: Block < 'a > > } , }
};
}
