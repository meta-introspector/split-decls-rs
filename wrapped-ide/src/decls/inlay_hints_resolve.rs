macro_rules! deps {
    () => {
        InlayHint!();
        InlayHintsConfig!();
        InlayHintCtx!();
    };
}

macro_rules! inlay_hints_resolve {
    () => {
        deps!();
        pub (crate) fn inlay_hints_resolve (db : & RootDatabase , file_id : FileId , resolve_range : TextRange , hash : u64 , config : & InlayHintsConfig < '_ > , hasher : impl Fn (& InlayHint) -> u64 ,) -> Option < InlayHint > { let _p = tracing :: info_span ! ("inlay_hints_resolve") . entered () ; let sema = Semantics :: new (db) ; let file_id = sema . attach_first_edition (file_id) . unwrap_or_else (| | EditionedFileId :: current_edition (db , file_id)) ; let file = sema . parse (file_id) ; let file = file . syntax () ; let scope = sema . scope (file) ? ; let famous_defs = FamousDefs (& sema , scope . krate ()) ; let mut acc = Vec :: new () ; let display_target = famous_defs . 1 . to_display_target (sema . db) ; let ctx = & mut InlayHintCtx :: default () ; let mut hints = | event | { if let Some (node) = handle_event (ctx , event) { hints (& mut acc , ctx , & famous_defs , config , file_id , display_target , node) ; } } ; let mut preorder = file . preorder () ; while let Some (event) = preorder . next () { if matches ! (& event , WalkEvent :: Enter (node) if resolve_range . intersect (node . text_range ()) . is_none ()) { preorder . skip_subtree () ; continue ; } hints (event) ; } acc . into_iter () . find (| hint | hasher (hint) == hash) }
    };
}

inlay_hints_resolve!();