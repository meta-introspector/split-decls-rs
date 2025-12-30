// Generated macro for impl_1440 (impl)
macro_rules! Depcrate_stringimpl_1440 {
() => {
// Module: crate::string
// Provides: {"impl_1440"}
// Dependencies: {}
impl < 'a , I : Iterator < Item = & 'a Hir > > Iterator for ConcatIter < 'a , I > { type Item = ParseResult < Vec < u8 > > ; fn next (& mut self) -> Option < Self :: Item > { if let Some (next) = self . next . take () { return Some (bytes_regex_parsed (next)) ; } while let Some (next) = self . iter . next () { match next . kind () { Literal (literal) => self . buf . extend_from_slice (& literal . 0) , _ => { return if ! self . buf . is_empty () { self . next = Some (next) ; flush_lit_buf (self) } else { Some (bytes_regex_parsed (next)) } ; } } } if ! self . buf . is_empty () { flush_lit_buf (self) } else { self . next . take () . map (bytes_regex_parsed) } } }
};
}
