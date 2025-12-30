// Generated macro for can_be_block_comment (function)
macro_rules! Depcrate_attrcan_be_block_comment {
() => {
// Module: crate::attr
// Provides: {"can_be_block_comment"}
// Dependencies: {}
fn can_be_block_comment (value : & str) -> bool { let mut depth = 0usize ; let bytes = value . as_bytes () ; let mut i = 0usize ; let upper = bytes . len () - 1 ; while i < upper { if bytes [i] == b'/' && bytes [i + 1] == b'*' { depth += 1 ; i += 2 ; } else if bytes [i] == b'*' && bytes [i + 1] == b'/' { if depth == 0 { return false ; } depth -= 1 ; i += 2 ; } else { i += 1 ; } } depth == 0 && ! value . ends_with ('/') }
};
}
