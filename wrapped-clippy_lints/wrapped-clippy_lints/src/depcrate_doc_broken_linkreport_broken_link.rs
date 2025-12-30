// Generated macro for report_broken_link (function)
macro_rules! Depcrate_doc_broken_linkreport_broken_link {
() => {
// Module: crate::doc::broken_link
// Provides: {"report_broken_link"}
// Dependencies: {}
fn report_broken_link (cx : & LateContext < '_ > , frag_span : Span , offset : usize) { let start = frag_span . lo () ; let end = start + BytePos :: from_usize (offset) ; let span = Span :: new (start , end , frag_span . ctxt () , frag_span . parent ()) ; span_lint (cx , DOC_BROKEN_LINK , span , "possible broken doc link: broken across multiple lines" ,) ; }
};
}
