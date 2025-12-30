// Generated macro for warn_if_broken_link (function)
macro_rules! Depcrate_doc_broken_linkwarn_if_broken_link {
() => {
// Module: crate::doc::broken_link
// Provides: {"warn_if_broken_link"}
// Dependencies: {}
fn warn_if_broken_link (cx : & LateContext < '_ > , bl : & PullDownBrokenLink < '_ > , doc : & str , fragments : & [DocFragment]) { let mut len = 0 ; let (_ , raw_link) = doc . split_at (bl . span . start) ; let raw_link = match raw_link . split_once (']') { None => return , Some ((prefix , suffix)) => { len += prefix . len () + 1 ; suffix } , } ; let raw_link = match raw_link . split_once ('(') { None => return , Some ((prefix , suffix)) => { if ! prefix . is_empty () { return ; } len += prefix . len () + 1 ; suffix } , } ; if raw_link . starts_with ("(http") { return ; } for c in raw_link . chars () { if c == ')' { return ; } if c == '\n' && let Some ((span , _)) = source_span_for_markdown_range (cx . tcx , doc , & bl . span , fragments) { report_broken_link (cx , span , len) ; break ; } len += 1 ; } }
};
}
