// Generated macro for check_str (function)
macro_rules! Depcrate_unicodecheck_str {
() => {
// Module: crate::unicode
// Provides: {"check_str"}
// Dependencies: {}
fn check_str (cx : & LateContext < '_ > , span : Span , id : HirId) { if ! span_is_local (span) { return ; } let string = snippet (cx , span , "") ; if string . chars () . any (| c | ['\u{200B}' , '\u{ad}' , '\u{2060}'] . contains (& c)) { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , INVISIBLE_CHARACTERS , span , "invisible character detected" , | diag | { diag . span_suggestion (span , "consider replacing the string with" , string . replace ('\u{200B}' , "\\u{200B}") . replace ('\u{ad}' , "\\u{AD}") . replace ('\u{2060}' , "\\u{2060}") , Applicability :: MachineApplicable ,) ; }) ; } if string . chars () . any (| c | c as u32 > 0x7F) { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , NON_ASCII_LITERAL , span , "literal non-ASCII character detected" , | diag | { diag . span_suggestion (span , "consider replacing the string with" , if is_lint_allowed (cx , UNICODE_NOT_NFC , id) { escape (string . chars ()) } else { escape (string . nfc ()) } , Applicability :: MachineApplicable ,) ; } ,) ; } if is_lint_allowed (cx , NON_ASCII_LITERAL , id) && string . chars () . zip (string . nfc ()) . any (| (a , b) | a != b) { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , UNICODE_NOT_NFC , span , "non-NFC Unicode sequence detected" , | diag | { diag . span_suggestion (span , "consider replacing the string with" , string . nfc () . collect :: < String > () , Applicability :: MachineApplicable ,) ; }) ; } }
};
}
