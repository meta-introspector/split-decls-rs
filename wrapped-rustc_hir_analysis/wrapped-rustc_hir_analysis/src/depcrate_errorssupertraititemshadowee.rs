// Generated macro for SupertraitItemShadowee (enum)
macro_rules! Depcrate_errorsSupertraitItemShadowee {
() => {
// Module: crate::errors
// Provides: {"SupertraitItemShadowee"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum SupertraitItemShadowee { # [note (hir_analysis_supertrait_item_shadowee)] Labeled { # [primary_span] span : Span , supertrait : Symbol , } , # [note (hir_analysis_supertrait_item_multiple_shadowee)] Several { # [primary_span] spans : MultiSpan , traits : DiagSymbolList , } , }
};
}
