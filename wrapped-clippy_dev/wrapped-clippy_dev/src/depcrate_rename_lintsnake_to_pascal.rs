// Generated macro for snake_to_pascal (function)
macro_rules! Depcrate_rename_lintsnake_to_pascal {
() => {
// Module: crate::rename_lint
// Provides: {"snake_to_pascal"}
// Dependencies: {}
fn snake_to_pascal (s : & str) -> String { let mut dst = Vec :: with_capacity (s . len ()) ; let mut iter = s . bytes () ; | | -> Option < () > { dst . push (iter . next () ? . to_ascii_uppercase ()) ; while let Some (c) = iter . next () { if c == b'_' { dst . push (iter . next () ? . to_ascii_uppercase ()) ; } else { dst . push (c) ; } } Some (()) } () ; String :: from_utf8 (dst) . unwrap () }
};
}
