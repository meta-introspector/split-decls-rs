// Generated macro for verify_case (function)
macro_rules! Depcrate_matches_match_str_case_mismatchverify_case {
() => {
// Module: crate::matches::match_str_case_mismatch
// Provides: {"verify_case"}
// Dependencies: {}
fn verify_case < 'a > (case_method : & 'a CaseMethod , arms : & 'a [Arm < '_ >]) -> Option < (Span , Symbol) > { let case_check = match case_method { CaseMethod :: LowerCase => | input : & str | -> bool { input . chars () . all (| c | c . to_lowercase () . next () == Some (c)) } , CaseMethod :: AsciiLowerCase => | input : & str | -> bool { ! input . chars () . any (| c | c . is_ascii_uppercase ()) } , CaseMethod :: UpperCase => | input : & str | -> bool { input . chars () . all (| c | c . to_uppercase () . next () == Some (c)) } , CaseMethod :: AsciiUppercase => | input : & str | -> bool { ! input . chars () . any (| c | c . is_ascii_lowercase ()) } , } ; for arm in arms { if let PatKind :: Expr (PatExpr { kind : PatExprKind :: Lit { lit , negated : false } , .. }) = arm . pat . kind && let LitKind :: Str (symbol , _) = lit . node && let input = symbol . as_str () && ! case_check (input) { return Some ((lit . span , symbol)) ; } } None }
};
}
