// Generated macro for RefPat (struct)
macro_rules! Depcrate_dereferenceRefPat {
() => {
// Module: crate::dereference
// Provides: {"RefPat"}
// Dependencies: {}
struct RefPat { # [doc = " Whether every usage of the binding is dereferenced."] always_deref : bool , # [doc = " The spans of all the ref bindings for this local."] spans : Vec < Span > , # [doc = " The applicability of this suggestion."] app : Applicability , # [doc = " All the replacements which need to be made."] replacements : Vec < (Span , String) > , # [doc = " The [`HirId`] that the lint should be emitted at."] hir_id : HirId , }
};
}
