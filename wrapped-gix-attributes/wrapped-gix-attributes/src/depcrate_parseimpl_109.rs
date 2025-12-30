// Generated macro for impl_109 (impl)
macro_rules! Depcrate_parseimpl_109 {
() => {
// Module: crate::parse
// Provides: {"impl_109"}
// Dependencies: {}
impl < 'a > Iterator for Lines < 'a > { type Item = Result < (Kind , Iter < 'a > , usize) , Error > ; fn next (& mut self) -> Option < Self :: Item > { fn skip_blanks (line : & BStr) -> & BStr { line . find_not_byteset (BLANKS) . map_or (line , | pos | & line [pos ..]) } for line in self . lines . by_ref () { self . line_no += 1 ; let line = skip_blanks (line . into ()) ; if line . first () == Some (& b'#') { continue ; } match parse_line (line , self . line_no) { None => continue , Some (res) => return Some (res) , } } None } }
};
}
