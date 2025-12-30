// Generated macro for moniker (function)
macro_rules! Depcrate_monikermoniker {
() => {
// Module: crate::moniker
// Provides: {"moniker"}
// Dependencies: {}
pub (crate) fn moniker (db : & RootDatabase , FilePosition { file_id , offset } : FilePosition ,) -> Option < RangeInfo < Vec < MonikerResult > > > { let sema = & Semantics :: new (db) ; let file = sema . parse_guess_edition (file_id) . syntax () . clone () ; let current_crate : hir :: Crate = crates_for (db , file_id) . pop () ? . into () ; let original_token = pick_best_token (file . token_at_offset (offset) , | kind | match kind { IDENT | INT_NUMBER | LIFETIME_IDENT | T ! [self] | T ! [super] | T ! [crate] | T ! [Self] | COMMENT => 2 , kind if kind . is_trivia () => 0 , _ => 1 , }) ? ; if let Some (doc_comment) = token_as_doc_comment (& original_token) { return doc_comment . get_definition_with_descend_at (sema , offset , | def , _ , _ | { let m = def_to_moniker (db , def , current_crate) ? ; Some (RangeInfo :: new (original_token . text_range () , vec ! [m])) }) ; } let navs = sema . descend_into_macros_exact (original_token . clone ()) . into_iter () . filter_map (| token | { IdentClass :: classify_token (sema , & token) . map (IdentClass :: definitions_no_ops) . map (| it | { it . into_iter () . flat_map (| def | def_to_moniker (sema . db , def , current_crate)) }) }) . flatten () . unique () . collect :: < Vec < _ > > () ; Some (RangeInfo :: new (original_token . text_range () , navs)) }
};
}
