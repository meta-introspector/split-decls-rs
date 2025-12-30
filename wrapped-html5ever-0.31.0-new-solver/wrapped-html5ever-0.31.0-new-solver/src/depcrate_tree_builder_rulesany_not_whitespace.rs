// Generated macro for any_not_whitespace (function)
macro_rules! Depcrate_tree_builder_rulesany_not_whitespace {
() => {
// Module: crate::tree_builder::rules
// Provides: {"any_not_whitespace"}
// Dependencies: {}
fn any_not_whitespace (x : & StrTendril) -> bool { x . chars () . any (| c | ! c . is_ascii_whitespace ()) }
};
}
