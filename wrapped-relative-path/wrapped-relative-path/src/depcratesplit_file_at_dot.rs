// Generated macro for split_file_at_dot (function)
macro_rules! Depcratesplit_file_at_dot {
() => {
// Module: crate
// Provides: {"split_file_at_dot"}
// Dependencies: {}
# [inline (always)] fn split_file_at_dot (input : & str) -> (Option < & str > , Option < & str >) { if input == PARENT_STR { return (Some (input) , None) ; } let mut iter = input . rsplitn (2 , STEM_SEP) ; let after = iter . next () ; let before = iter . next () ; if before == Some ("") { (Some (input) , None) } else { (before , after) } }
};
}
