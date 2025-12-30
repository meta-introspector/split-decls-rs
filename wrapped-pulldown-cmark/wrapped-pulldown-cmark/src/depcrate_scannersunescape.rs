// Generated macro for unescape (function)
macro_rules! Depcrate_scannersunescape {
() => {
// Module: crate::scanners
// Provides: {"unescape"}
// Dependencies: {}
pub (crate) fn unescape < 'a , I : Into < CowStr < 'a > > > (input : I , is_in_table : bool) -> CowStr < 'a > { let input = input . into () ; let mut result = String :: new () ; let mut mark = 0 ; let mut i = 0 ; let bytes = input . as_bytes () ; while i < bytes . len () { match bytes [i ..] { [b'\\' , b'\\' , b'|' , ..] if is_in_table => { result . push_str (& input [mark .. i]) ; mark = i + 2 ; i += 3 ; } [b'\\' , cx , ..] if is_ascii_punctuation (cx) => { result . push_str (& input [mark .. i]) ; mark = i + 1 ; i += 2 ; } [b'&' , ..] => match scan_entity (& bytes [i ..]) { (n , Some (value)) => { result . push_str (& input [mark .. i]) ; result . push_str (& value) ; i += n ; mark = i ; } _ => i += 1 , } , [b'\r' , ..] => { result . push_str (& input [mark .. i]) ; i += 1 ; mark = i ; } _ => i += 1 , } } if mark == 0 { input } else { result . push_str (& input [mark ..]) ; result . into () } }
};
}
