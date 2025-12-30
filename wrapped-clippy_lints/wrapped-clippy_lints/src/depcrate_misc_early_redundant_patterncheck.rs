// Generated macro for check (function)
macro_rules! Depcrate_misc_early_redundant_patterncheck {
() => {
// Module: crate::misc_early::redundant_pattern
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & EarlyContext < '_ > , pat : & Pat) { if let PatKind :: Ident (ann , ident , Some (ref right)) = pat . kind && let PatKind :: Wild = right . kind { span_lint_and_sugg (cx , REDUNDANT_PATTERN , pat . span , format ! ("the `{} @ _` pattern can be written as just `{}`" , ident . name , ident . name ,) , "try" , format ! ("{}{}" , ann . prefix_str () , ident . name) , Applicability :: MachineApplicable ,) ; } }
};
}
