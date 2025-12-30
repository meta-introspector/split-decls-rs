// Generated macro for truncate_to_character_boundary (function)
macro_rules! Depcrate_reporttruncate_to_character_boundary {
() => {
// Module: crate::report
// Provides: {"truncate_to_character_boundary"}
// Dependencies: {}
fn truncate_to_character_boundary (s : & mut String , max_len : usize) { let mut boundary = cmp :: min (max_len , s . len ()) ; while ! s . is_char_boundary (boundary) { boundary -= 1 ; } s . truncate (boundary) ; }
};
}
