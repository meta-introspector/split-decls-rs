// Generated macro for escape_special_characters (function)
macro_rules! Depcrate_text_modificationsescape_special_characters {
() => {
// Module: crate::text_modifications
// Provides: {"escape_special_characters"}
// Dependencies: {}
pub (crate) fn escape_special_characters < 'a > (t : & 'a str , state : & State < 'a > , options : & Options < 'a >) -> Cow < 'a , str > { if state . is_in_code_block () || t . is_empty () { return Cow :: Borrowed (t) ; } let first = t . chars () . next () . expect ("at least one char") ; let first_special = options . special_characters () . contains (first) ; let ends_with_special = (state . next_is_link_like && t . ends_with ("!")) || (state . current_heading . is_some () && t . ends_with ("#")) ; let table_contains_pipe = ! state . table_alignments . is_empty () && t . contains ("|") ; if first_special || ends_with_special || table_contains_pipe { let mut s = String :: with_capacity (t . len () + 1) ; for (i , c) in t . char_indices () { if (i == 0 && first_special) || (i == t . len () - 1 && ends_with_special) || (c == '|' && table_contains_pipe) { s . push ('\\') ; } s . push (c) ; } Cow :: Owned (s) } else { Cow :: Borrowed (t) } }
};
}
