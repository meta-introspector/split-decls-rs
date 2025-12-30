// Generated macro for MatchArmWithNoBody (struct)
macro_rules! Depcrate_errorsMatchArmWithNoBody {
() => {
// Module: crate::errors
// Provides: {"MatchArmWithNoBody"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_match_arm_with_no_body)] pub (crate) struct MatchArmWithNoBody { # [primary_span] pub span : Span , # [suggestion (code = " => {{ todo!() }}" , applicability = "has-placeholders" , style = "verbose")] pub suggestion : Span , }
};
}
