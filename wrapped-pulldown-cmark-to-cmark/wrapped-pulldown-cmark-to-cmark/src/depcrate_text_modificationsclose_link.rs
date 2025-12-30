// Generated macro for close_link (function)
macro_rules! Depcrate_text_modificationsclose_link {
() => {
// Module: crate::text_modifications
// Provides: {"close_link"}
// Dependencies: {}
pub (crate) fn close_link < F > (uri : & str , title : & str , f : & mut F , link_type : LinkType) -> fmt :: Result where F : fmt :: Write , { let needs_brackets = { let mut depth = 0 ; for b in uri . bytes () { match b { b'(' => depth += 1 , b')' => depth -= 1 , b' ' => { depth += 1 ; break ; } _ => { } } if depth > 3 { break ; } } depth != 0 } ; let separator = match link_type { LinkType :: Shortcut => ": " , _ => "(" , } ; if needs_brackets { write ! (f , "]{separator}<{uri}>") ? ; } else { write ! (f , "]{separator}{uri}") ? ; } if ! title . is_empty () { write ! (f , " \"{title}\"" , title = EscapeLinkTitle (title)) ? ; } if link_type != LinkType :: Shortcut { f . write_char (')') ? ; } Ok (()) }
};
}
