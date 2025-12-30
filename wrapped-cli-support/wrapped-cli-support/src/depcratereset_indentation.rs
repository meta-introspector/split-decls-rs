// Generated macro for reset_indentation (function)
macro_rules! Depcratereset_indentation {
() => {
// Module: crate
// Provides: {"reset_indentation"}
// Dependencies: {}
fn reset_indentation (s : & str) -> String { let mut indent : u32 = 0 ; let mut dst = String :: new () ; fn is_doc_comment (line : & str) -> bool { line . starts_with ("*") } for line in s . lines () { let line = line . trim () ; if is_doc_comment (line) { for _ in 0 .. indent { dst . push_str ("    ") ; } dst . push (' ') ; dst . push_str (line) ; dst . push ('\n') ; continue ; } if line . starts_with ('}') { indent = indent . saturating_sub (1) ; } let extra = if line . starts_with (':') || line . starts_with ('?') { 1 } else { 0 } ; if ! line . is_empty () { for _ in 0 .. indent + extra { dst . push_str ("    ") ; } dst . push_str (line) ; } dst . push ('\n') ; if line . ends_with ('{') { indent += 1 ; } } dst }
};
}
