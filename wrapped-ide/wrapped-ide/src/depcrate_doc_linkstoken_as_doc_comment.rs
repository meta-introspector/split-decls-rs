// Generated macro for token_as_doc_comment (function)
macro_rules! Depcrate_doc_linkstoken_as_doc_comment {
() => {
// Module: crate::doc_links
// Provides: {"token_as_doc_comment"}
// Dependencies: {}
pub (crate) fn token_as_doc_comment (doc_token : & SyntaxToken) -> Option < DocCommentToken > { (match_ast ! { match doc_token { ast :: Comment (comment) => TextSize :: try_from (comment . prefix () . len ()) . ok () , ast :: String (string) => { doc_token . parent_ancestors () . find_map (ast :: Attr :: cast) . filter (| attr | attr . simple_name () . as_deref () == Some ("doc")) ?; if doc_token . parent_ancestors () . find_map (ast :: MacroCall :: cast) . filter (| mac | mac . path () . and_then (| p | p . segment () ?. name_ref ()) . as_ref () . map (| n | n . text ()) . as_deref () == Some ("include_str")) . is_some () { return None ; } string . open_quote_text_range () . map (| it | it . len ()) } , _ => None , } }) . map (| prefix_len | DocCommentToken { prefix_len , doc_token : doc_token . clone () }) }
};
}
