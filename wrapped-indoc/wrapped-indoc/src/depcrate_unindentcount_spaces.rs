// Generated macro for count_spaces (function)
macro_rules! Depcrate_unindentcount_spaces {
() => {
// Module: crate::unindent
// Provides: {"count_spaces"}
// Dependencies: {}
fn count_spaces (line : & [u8]) -> Option < usize > { for (i , ch) in line . iter () . enumerate () { if * ch != b' ' && * ch != b'\t' { return Some (i) ; } } None }
};
}
