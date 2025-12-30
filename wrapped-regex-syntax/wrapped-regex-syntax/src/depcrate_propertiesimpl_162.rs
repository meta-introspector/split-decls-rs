// Generated macro for impl_162 (impl)
macro_rules! Depcrate_propertiesimpl_162 {
() => {
// Module: crate::properties
// Provides: {"impl_162"}
// Dependencies: {}
impl Arbitrary for RegexLikeString { fn arbitrary < G : Gen > (g : & mut G) -> RegexLikeString { const SPECIAL : & 'static [char] = & ['\\' , '.' , '+' , '*' , '?' , '(' , ')' , '|' , '[' , ']' , '{' , '}' , '^' , '$' ,] ; let size = { let s = g . size () ; g . gen_range (0 , s) } ; RegexLikeString ((0 .. size) . map (| _ | { if g . gen_weighted_bool (3) { * g . choose (SPECIAL) . unwrap () } else { g . gen () } }) . collect ()) } fn shrink (& self) -> Box < Iterator < Item = RegexLikeString > > { Box :: new (self . 0 . shrink () . map (RegexLikeString)) } }
};
}
