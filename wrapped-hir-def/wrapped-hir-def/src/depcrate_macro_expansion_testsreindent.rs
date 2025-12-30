// Generated macro for reindent (function)
macro_rules! Depcrate_macro_expansion_testsreindent {
() => {
// Module: crate::macro_expansion_tests
// Provides: {"reindent"}
// Dependencies: {}
fn reindent (indent : IndentLevel , pp : String) -> String { if ! pp . contains ('\n') { return pp ; } let mut lines = pp . split_inclusive ('\n') ; let mut res = lines . next () . unwrap () . to_owned () ; for line in lines { if line . trim () . is_empty () { res . push_str (line) } else { format_to ! (res , "{}{}" , indent , line) } } res }
};
}
