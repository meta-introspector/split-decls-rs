// Generated macro for extract_doc_comments (function)
macro_rules! Depcrate_parserextract_doc_comments {
() => {
// Module: crate::parser
// Provides: {"extract_doc_comments"}
// Dependencies: {}
# [doc = " Extract the documentation comments from a Vec of attributes"] fn extract_doc_comments (attrs : & [syn :: Attribute]) -> Vec < String > { attrs . iter () . filter_map (| a | { if a . path () . segments . iter () . any (| s | s . ident == "doc") { let tokens = match & a . meta { syn :: Meta :: Path (_) => None , syn :: Meta :: List (list) => Some (list . tokens . clone ()) , syn :: Meta :: NameValue (name_value) => Some (name_value . value . to_token_stream ()) , } ; Some (tokens . into_iter () . flatten () . filter_map (| t | match t { TokenTree :: Literal (lit) => { let quoted = lit . to_string () ; Some (try_unescape (& quoted) . unwrap_or (quoted)) } _ => None , }) ,) } else { None } }) . fold (vec ! [] , | mut acc , a | { acc . extend (a) ; acc }) }
};
}
