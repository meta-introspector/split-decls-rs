// Generated macro for snippet_eq_ty (function)
macro_rules! Depcrate_casts_unnecessary_castsnippet_eq_ty {
() => {
// Module: crate::casts::unnecessary_cast
// Provides: {"snippet_eq_ty"}
// Dependencies: {}
fn snippet_eq_ty (snippet : & str , ty : Ty < '_ >) -> bool { snippet . trim () == ty . to_string () || snippet . trim () . contains (& format ! ("::{ty}")) }
};
}
