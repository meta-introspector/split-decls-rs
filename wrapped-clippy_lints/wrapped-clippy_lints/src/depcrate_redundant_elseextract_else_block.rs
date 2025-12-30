// Generated macro for extract_else_block (function)
macro_rules! Depcrate_redundant_elseextract_else_block {
() => {
// Module: crate::redundant_else
// Provides: {"extract_else_block"}
// Dependencies: {}
fn extract_else_block (mut block : & str) -> String { block = block . strip_prefix ("{") . unwrap_or (block) ; block = block . strip_suffix ("}") . unwrap_or (block) ; block . trim_end () . to_string () }
};
}
