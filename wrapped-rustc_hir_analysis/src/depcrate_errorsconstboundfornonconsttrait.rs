// Generated macro for ConstBoundForNonConstTrait (struct)
macro_rules! Depcrate_errorsConstBoundForNonConstTrait {
() => {
// Module: crate::errors
// Provides: {"ConstBoundForNonConstTrait"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_const_bound_for_non_const_trait)] pub (crate) struct ConstBoundForNonConstTrait { # [primary_span] # [label] pub span : Span , pub modifier : & 'static str , # [note] pub def_span : Option < Span > , pub suggestion_pre : & 'static str , # [suggestion (applicability = "machine-applicable" , code = "#[const_trait] " , style = "verbose")] pub suggestion : Option < Span > , pub trait_name : String , }
};
}
