// Generated macro for lint_unneeded_unit_return (function)
macro_rules! Depcrate_unused_unitlint_unneeded_unit_return {
() => {
// Module: crate::unused_unit
// Provides: {"lint_unneeded_unit_return"}
// Dependencies: {}
fn lint_unneeded_unit_return (cx : & LateContext < '_ > , ty_span : Span , span : Span) { let (ret_span , appl) = if let Some (Some (rpos)) = span . with_hi (ty_span . hi ()) . with_source_text (cx , position_before_rarrow) { (ty_span . with_lo (span . lo () + BytePos :: from_usize (rpos)) , Applicability :: MachineApplicable ,) } else { (ty_span , Applicability :: MaybeIncorrect) } ; span_lint_and_sugg (cx , UNUSED_UNIT , ret_span , "unneeded unit return type" , "remove the `-> ()`" , String :: new () , appl ,) ; }
};
}
