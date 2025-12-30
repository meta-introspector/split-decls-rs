// Generated macro for impl_102 (impl)
macro_rules! Depcrate_syntax_textimpl_102 {
() => {
// Module: crate::syntax_text
// Provides: {"impl_102"}
// Dependencies: {}
impl PartialEq < str > for SyntaxText { fn eq (& self , mut rhs : & str) -> bool { self . try_for_each_chunk (| chunk | { if ! rhs . starts_with (chunk) { return Err (()) ; } rhs = & rhs [chunk . len () ..] ; Ok (()) }) . is_ok () && rhs . is_empty () } }
};
}
