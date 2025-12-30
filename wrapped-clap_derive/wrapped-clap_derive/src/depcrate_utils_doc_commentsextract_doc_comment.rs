// Generated macro for extract_doc_comment (function)
macro_rules! Depcrate_utils_doc_commentsextract_doc_comment {
() => {
// Module: crate::utils::doc_comments
// Provides: {"extract_doc_comment"}
// Dependencies: {}
pub (crate) fn extract_doc_comment (attrs : & [syn :: Attribute]) -> Vec < String > { let mut lines : Vec < _ > = attrs . iter () . filter (| attr | attr . path () . is_ident ("doc")) . filter_map (| attr | { match & attr . meta { syn :: Meta :: NameValue (syn :: MetaNameValue { value : syn :: Expr :: Lit (syn :: ExprLit { lit : syn :: Lit :: Str (s) , .. }) , .. }) => Some (s . value ()) , _ => None , } }) . skip_while (| s | is_blank (s)) . flat_map (| s | { let lines = s . split ('\n') . map (| s | { let s = s . strip_prefix (' ') . unwrap_or (s) ; s . to_owned () }) . collect :: < Vec < _ > > () ; lines }) . collect () ; while let Some (true) = lines . last () . map (| s | is_blank (s)) { lines . pop () ; } lines }
};
}
