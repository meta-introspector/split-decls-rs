// Generated macro for check (function)
macro_rules! Depcrate_doc_markdowncheck {
() => {
// Module: crate::doc::markdown
// Provides: {"check"}
// Dependencies: {}
pub fn check (cx : & LateContext < '_ > , valid_idents : & FxHashSet < String > , text : & str , fragments : & Fragments < '_ > , fragment_range : Range < usize > , code_level : isize , blockquote_level : isize ,) { for orig_word in text . split (| c : char | c . is_whitespace () || c == '\'') { let trim_pattern = | c : char | ! c . is_alphanumeric () && c != ':' ; let mut word = orig_word . trim_end_matches (trim_pattern) ; if let Some (tmp_word) = orig_word . get (.. word . len () + 2) && tmp_word . ends_with ("()") { word = tmp_word ; } let original_len = word . len () ; word = word . trim_start_matches (trim_pattern) ; if word . starts_with (':') && ! word . starts_with ("::") { word = word . trim_start_matches (':') ; } if word . ends_with (':') && ! word . ends_with ("::") { word = word . trim_end_matches (':') ; } if valid_idents . contains (word) || word . chars () . all (| c | c == ':') { continue ; } let size_diff = original_len - word . len () ; let mut open_parens = 0 ; let mut close_parens = 0 ; for c in word . chars () { if c == '(' { open_parens += 1 ; } else if c == ')' { close_parens += 1 ; } } while close_parens < open_parens && let Some (tmp_word) = orig_word . get (size_diff ..= (word . len () + size_diff)) && tmp_word . ends_with (')') { word = tmp_word ; close_parens += 1 ; } let fragment_offset = word . as_ptr () as usize - text . as_ptr () as usize ; check_word (cx , word , fragments , & fragment_range , fragment_offset , code_level , blockquote_level ,) ; } }
};
}
