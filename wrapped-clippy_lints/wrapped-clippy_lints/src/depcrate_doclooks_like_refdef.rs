// Generated macro for looks_like_refdef (function)
macro_rules! Depcrate_doclooks_like_refdef {
() => {
// Module: crate::doc
// Provides: {"looks_like_refdef"}
// Dependencies: {}
fn looks_like_refdef (doc : & str , range : Range < usize >) -> Option < Range < usize > > { if range . end < range . start { return None ; } let offset = range . start ; let mut iterator = doc . as_bytes () [range] . iter () . copied () . enumerate () ; let mut start = None ; while let Some ((i , byte)) = iterator . next () { match byte { b'\\' => { iterator . next () ; } , b'[' => { start = Some (i + offset) ; } , b']' if let Some (start) = start && doc . as_bytes () . get (i + offset + 1) == Some (& b':') => { return Some (start .. i + offset + 1) ; } , _ => { } , } } None }
};
}
