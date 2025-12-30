// Generated macro for impl_9 (impl)
macro_rules! Depcrate_parseimpl_9 {
() => {
// Module: crate::parse
// Provides: {"impl_9"}
// Dependencies: {}
impl < 'a > Iterator for Lines < 'a > { type Item = Result < Entry < 'a > , Error > ; fn next (& mut self) -> Option < Self :: Item > { for line in self . lines . by_ref () { self . line_no += 1 ; match line . first () { None => continue , Some (b) if * b == b'#' => continue , Some (_) => { } } let line = line . trim () ; if line . is_empty () { continue ; } return parse_line (line . into () , self . line_no) . into () ; } None } }
};
}
