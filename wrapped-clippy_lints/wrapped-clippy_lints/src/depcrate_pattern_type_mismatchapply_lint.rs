// Generated macro for apply_lint (function)
macro_rules! Depcrate_pattern_type_mismatchapply_lint {
() => {
// Module: crate::pattern_type_mismatch
// Provides: {"apply_lint"}
// Dependencies: {}
fn apply_lint (cx : & LateContext < '_ > , pat : & Pat < '_ > , deref_possible : DerefPossible) -> bool { let maybe_mismatch = find_first_mismatch (cx , pat) ; if let Some ((span , mutability , level)) = maybe_mismatch { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , PATTERN_TYPE_MISMATCH , span , "type of pattern does not match the expression type" , | diag | { diag . help (format ! ("{}explicitly match against a `{}` pattern and adjust the enclosed variable bindings" , match (deref_possible , level) { (DerefPossible :: Possible , Level :: Top) => "use `*` to dereference the match expression or " , _ => "" , } , match mutability { Mutability :: Mut => "&mut _" , Mutability :: Not => "&_" , } ,)) ; } ,) ; true } else { false } }
};
}
