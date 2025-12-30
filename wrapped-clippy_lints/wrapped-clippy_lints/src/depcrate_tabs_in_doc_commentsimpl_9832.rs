// Generated macro for impl_9832 (impl)
macro_rules! Depcrate_tabs_in_doc_commentsimpl_9832 {
() => {
// Module: crate::tabs_in_doc_comments
// Provides: {"impl_9832"}
// Dependencies: {}
impl TabsInDocComments { fn warn_if_tabs_in_doc (cx : & EarlyContext < '_ > , attr : & ast :: Attribute) { if let ast :: AttrKind :: DocComment (_ , comment) = attr . kind { let comment = comment . as_str () ; for (lo , hi) in get_chunks_of_tabs (comment) { let new_span = Span :: new (attr . span . lo () + BytePos (3 + lo) , attr . span . lo () + BytePos (3 + hi) , attr . span . ctxt () , attr . span . parent () ,) ; span_lint_and_sugg (cx , TABS_IN_DOC_COMMENTS , new_span , "using tabs in doc comments is not recommended" , "consider using four spaces per tab" , "    " . repeat ((hi - lo) as usize) , Applicability :: MaybeIncorrect ,) ; } } } }
};
}
