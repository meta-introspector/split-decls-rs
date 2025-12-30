// Generated macro for strip_shebang (function)
macro_rules! Depcrate_frontmatterstrip_shebang {
() => {
// Module: crate::frontmatter
// Provides: {"strip_shebang"}
// Dependencies: {}
# [doc = " Returns the index after the shebang line, if present"] pub fn strip_shebang (input : & str) -> Option < usize > { if let Some (rest) = input . strip_prefix ("#!") { if ! rest . trim_start () . starts_with ('[') { let newline_end = input . find ('\n') . map (| pos | pos + 1) . unwrap_or (input . len ()) ; return Some (newline_end) ; } } None }
};
}
