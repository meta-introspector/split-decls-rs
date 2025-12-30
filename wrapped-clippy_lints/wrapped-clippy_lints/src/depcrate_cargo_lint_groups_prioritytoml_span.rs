// Generated macro for toml_span (function)
macro_rules! Depcrate_cargo_lint_groups_prioritytoml_span {
() => {
// Module: crate::cargo::lint_groups_priority
// Provides: {"toml_span"}
// Dependencies: {}
fn toml_span (range : Range < usize > , file : & SourceFile) -> Span { Span :: new (file . start_pos + BytePos :: from_usize (range . start) , file . start_pos + BytePos :: from_usize (range . end) , SyntaxContext :: root () , None ,) }
};
}
