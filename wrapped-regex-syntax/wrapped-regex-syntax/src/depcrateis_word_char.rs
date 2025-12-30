// Generated macro for is_word_char (function)
macro_rules! Depcrateis_word_char {
() => {
// Module: crate
// Provides: {"is_word_char"}
// Dependencies: {}
# [doc = " Returns true if and only if `c` is a word character."] # [doc (hidden)] pub fn is_word_char (c : char) -> bool { match c { '_' | '0' ..= '9' | 'a' ..= 'z' | 'A' ..= 'Z' => true , _ => :: unicode :: regex :: PERLW . binary_search_by (| & (start , end) | { if c >= start && c <= end { Ordering :: Equal } else if start > c { Ordering :: Greater } else { Ordering :: Less } }) . is_ok () , } }
};
}
