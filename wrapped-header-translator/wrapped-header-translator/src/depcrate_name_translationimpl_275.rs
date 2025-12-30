// Generated macro for impl_275 (impl)
macro_rules! Depcrate_name_translationimpl_275 {
() => {
// Module: crate::name_translation
// Provides: {"impl_275"}
// Dependencies: {}
impl < 'a > Iterator for Iter < 'a > { type Item = & 'a str ; fn next (& mut self) -> Option < Self :: Item > { if self . remaining . is_empty () { return None ; } let remaining = | idx | { let (_ , remaining) = self . remaining . split_at (idx) ; remaining } ; fn skip (s : & str , f : impl Fn (char) -> bool) -> usize { s . find (| c : char | ! f (c)) . unwrap_or (s . len ()) } if self . remaining . starts_with ("_") { let (current , remaining) = self . remaining . split_at (1) ; self . remaining = remaining ; return Some (current) ; } let mut idx = 0 ; idx += skip (remaining (idx) , | c | c . is_ascii_uppercase ()) ; if idx > 1 { let next = skip (remaining (idx) , | c | c . is_ascii_lowercase ()) ; if remaining (idx) . is_empty () { } else if is_plural_suffix (remaining (idx) . split_at (next) . 0) && remaining (idx - 1) . split_at (next + 1) . 0 != "Is" { idx += next ; } else if remaining (idx) . as_bytes () [0] . is_ascii_lowercase () { idx -= 1 ; } let (current , remaining) = self . remaining . split_at (idx) ; self . remaining = remaining ; return Some (current) ; } idx += skip (remaining (idx) , | c | ! c . is_ascii_uppercase () && c != '_') ; let (current , remaining) = self . remaining . split_at (idx) ; self . remaining = remaining ; Some (current) } }
};
}
