// Generated macro for impl_2159 (impl)
macro_rules! Depcrate_field_scoped_visibility_modifiersimpl_2159 {
() => {
// Module: crate::field_scoped_visibility_modifiers
// Provides: {"impl_2159"}
// Dependencies: {}
impl EarlyLintPass for FieldScopedVisibilityModifiers { fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & Item) { let ItemKind :: Struct (_ , _ , ref st) = item . kind else { return ; } ; for field in st . fields () { let VisibilityKind :: Restricted { path , .. } = & field . vis . kind else { continue ; } ; if ! path . segments . is_empty () && path . segments [0] . ident . name == rustc_span :: symbol :: kw :: SelfLower { continue ; } # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , FIELD_SCOPED_VISIBILITY_MODIFIERS , field . vis . span , "scoped visibility modifier on a field" , | diag | { diag . help ("consider making the field private and adding a scoped visibility method for it") ; } ,) ; } } }
};
}
