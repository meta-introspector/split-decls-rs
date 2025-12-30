// Generated macro for impl_30 (impl)
macro_rules! Depcrate_commit_message_bodyimpl_30 {
() => {
// Module: crate::commit::message::body
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a > Iterator for Trailers < 'a > { type Item = TrailerRef < 'a > ; fn next (& mut self) -> Option < Self :: Item > { if self . cursor . is_empty () { return None ; } for mut line in self . cursor . lines_with_terminator () { self . cursor = & self . cursor [line . len () ..] ; if let Some (trailer) = terminated (parse_single_line_trailer :: < () > , eof) . parse_next (& mut line) . ok () . map (| (token , value) | TrailerRef { token : token . trim () . as_bstr () , value : value . trim () . as_bstr () , }) { return Some (trailer) ; } } None } }
};
}
