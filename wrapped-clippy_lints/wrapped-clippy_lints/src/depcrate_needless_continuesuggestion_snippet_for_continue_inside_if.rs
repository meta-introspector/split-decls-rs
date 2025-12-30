// Generated macro for suggestion_snippet_for_continue_inside_if (function)
macro_rules! Depcrate_needless_continuesuggestion_snippet_for_continue_inside_if {
() => {
// Module: crate::needless_continue
// Provides: {"suggestion_snippet_for_continue_inside_if"}
// Dependencies: {}
fn suggestion_snippet_for_continue_inside_if (cx : & LateContext < '_ > , data : & LintData < '_ >) -> String { let mut applicability = Applicability :: MachineApplicable ; let (cond_code , _) = snippet_with_context (cx , data . if_cond . span , data . if_expr . span . ctxt () , ".." , & mut applicability ,) ; let continue_code = snippet_block (cx , data . if_block . span , ".." , Some (data . if_expr . span)) ; let else_code = snippet_block (cx , data . else_expr . span , ".." , Some (data . if_expr . span)) ; let indent_if = indent_of (cx , data . if_expr . span) . unwrap_or (0) ; format ! ("{indent}if {cond_code} {continue_code}\n{indent}{else_code}" , indent = " " . repeat (indent_if) ,) }
};
}
