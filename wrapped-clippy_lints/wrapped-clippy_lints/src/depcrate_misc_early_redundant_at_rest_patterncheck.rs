// Generated macro for check (function)
macro_rules! Depcrate_misc_early_redundant_at_rest_patterncheck {
() => {
// Module: crate::misc_early::redundant_at_rest_pattern
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & EarlyContext < '_ > , pat : & Pat) { if ! pat . span . in_external_macro (cx . sess () . source_map ()) && let PatKind :: Slice (slice) = & pat . kind && let [one] = & * * slice && let PatKind :: Ident (annotation , ident , Some (rest)) = & one . kind && let PatKind :: Rest = rest . kind { span_lint_and_sugg (cx , REDUNDANT_AT_REST_PATTERN , pat . span , "using a rest pattern to bind an entire slice to a local" , "this is better represented with just the binding" , format ! ("{}{ident}" , annotation . prefix_str ()) , Applicability :: MachineApplicable ,) ; } }
};
}
