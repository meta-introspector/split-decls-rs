// Generated macro for SupertraitItemShadowing (struct)
macro_rules! Depcrate_errorsSupertraitItemShadowing {
() => {
// Module: crate::errors
// Provides: {"SupertraitItemShadowing"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (hir_analysis_supertrait_item_shadowing)] pub (crate) struct SupertraitItemShadowing { pub item : Symbol , pub subtrait : Symbol , # [subdiagnostic] pub shadowee : SupertraitItemShadowee , }
};
}
