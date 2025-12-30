// Generated macro for TypeofReservedKeywordUsed (struct)
macro_rules! Depcrate_errorsTypeofReservedKeywordUsed {
() => {
// Module: crate::errors
// Provides: {"TypeofReservedKeywordUsed"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_typeof_reserved_keyword_used , code = E0516)] pub (crate) struct TypeofReservedKeywordUsed < 'tcx > { pub ty : Ty < 'tcx > , # [primary_span] # [label] pub span : Span , # [suggestion (style = "verbose" , code = "{ty}")] pub opt_sugg : Option < (Span , Applicability) > , }
};
}
