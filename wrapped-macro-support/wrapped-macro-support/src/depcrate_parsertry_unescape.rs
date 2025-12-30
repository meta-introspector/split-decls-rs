// Generated macro for try_unescape (function)
macro_rules! Depcrate_parsertry_unescape {
() => {
// Module: crate::parser
// Provides: {"try_unescape"}
// Dependencies: {}
fn try_unescape (mut s : & str) -> Option < String > { s = s . strip_prefix ('"') . unwrap_or (s) ; s = s . strip_suffix ('"') . unwrap_or (s) ; let mut result = String :: with_capacity (s . len ()) ; let mut chars = s . chars () ; while let Some (c) = chars . next () { if c == '\\' { let c = chars . next () ? ; match c { 't' => result . push ('\t') , 'r' => result . push ('\r') , 'n' => result . push ('\n') , '\\' | '\'' | '"' => result . push (c) , 'u' => { if chars . next () != Some ('{') { return None ; } let (c , next) = unescape_unicode (& mut chars) ? ; result . push (c) ; if next != '}' { return None ; } } _ => return None , } } else { result . push (c) ; } } Some (result) }
};
}
