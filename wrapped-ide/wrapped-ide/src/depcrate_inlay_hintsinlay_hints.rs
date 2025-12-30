// Generated macro for inlay_hints (function)
macro_rules! Depcrate_inlay_hintsinlay_hints {
() => {
// Module: crate::inlay_hints
// Provides: {"inlay_hints"}
// Dependencies: {}
pub (crate) fn inlay_hints (db : & RootDatabase , file_id : FileId , range_limit : Option < TextRange > , config : & InlayHintsConfig < '_ > ,) -> Vec < InlayHint > { let _p = tracing :: info_span ! ("inlay_hints") . entered () ; let sema = Semantics :: new (db) ; let file_id = sema . attach_first_edition (file_id) . unwrap_or_else (| | EditionedFileId :: current_edition (db , file_id)) ; let file = sema . parse (file_id) ; let file = file . syntax () ; let mut acc = Vec :: new () ; let Some (scope) = sema . scope (file) else { return acc ; } ; let famous_defs = FamousDefs (& sema , scope . krate ()) ; let display_target = famous_defs . 1 . to_display_target (sema . db) ; let ctx = & mut InlayHintCtx :: default () ; let mut hints = | event | { if let Some (node) = handle_event (ctx , event) { hints (& mut acc , ctx , & famous_defs , config , file_id , display_target , node) ; } } ; let mut preorder = file . preorder () ; hir :: attach_db (sema . db , | | { while let Some (event) = preorder . next () { if matches ! ((& event , range_limit) , (WalkEvent :: Enter (node) , Some (range)) if range . intersect (node . text_range ()) . is_none ()) { preorder . skip_subtree () ; continue ; } hints (event) ; } }) ; if let Some (range_limit) = range_limit { acc . retain (| hint | range_limit . contains_range (hint . range)) ; } acc }
};
}
