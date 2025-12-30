// Generated macro for indent (function)
macro_rules! Depcrateindent {
() => {
// Module: crate
// Provides: {"indent"}
// Dependencies: {}
fn indent (code : & str) -> Source { let mut indented = Source :: default () ; let mut was_empty = false ; for line in code . lines () { let trimmed = line . trim () ; if trimmed . is_empty () { if was_empty { continue ; } was_empty = true ; } else { was_empty = false ; } if trimmed . starts_with ('}') { indented . deindent (2) } indented . push_str (trimmed) ; if trimmed . ends_with ('{') && ! trimmed . starts_with ("///") { indented . indent (2) } indented . push_str ("\n") ; } indented }
};
}
