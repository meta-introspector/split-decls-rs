// Generated macro for check_missing_else (function)
macro_rules! Depcrate_formattingcheck_missing_else {
() => {
// Module: crate::formatting
// Provides: {"check_missing_else"}
// Dependencies: {}
fn check_missing_else (cx : & EarlyContext < '_ > , first : & Expr , second : & Expr) { if ! first . span . from_expansion () && ! second . span . from_expansion () && matches ! (first . kind , ExprKind :: If (..)) && (is_block (second) || is_if (second)) && is_span_if (cx , first . span) && let else_span = first . span . between (second . span) && let Some (else_snippet) = snippet_opt (cx , else_span) && ! else_snippet . chars () . any (| c | c == '\n' || ! c . is_whitespace ()) { let (looks_like , next_thing) = if is_if (second) { ("an `else if`" , "the second `if`") } else { ("an `else {..}`" , "the next block") } ; span_lint_and_note (cx , POSSIBLE_MISSING_ELSE , else_span , format ! ("this looks like {looks_like} but the `else` is missing") , None , format ! ("to remove this lint, add the missing `else` or add a new line before {next_thing}" ,) ,) ; } }
};
}
