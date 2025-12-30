// Generated macro for by_name (function)
macro_rules! Depcrate_unicodeby_name {
() => {
// Module: crate::unicode
// Provides: {"by_name"}
// Dependencies: {}
pub fn by_name (name : & str) -> Option < Box < dyn Fn (char) -> bool > > { for property in binary :: BY_NAME { if name == property . 0 . to_uppercase () { return Some (Box :: new (move | c | property . 1 . contains_char (c))) ; } } for property in category :: BY_NAME { if name == property . 0 . to_uppercase () { return Some (Box :: new (move | c | property . 1 . contains_char (c))) ; } } for property in script :: BY_NAME { if name == property . 0 . to_uppercase () { return Some (Box :: new (move | c | property . 1 . contains_char (c))) ; } } None }
};
}
