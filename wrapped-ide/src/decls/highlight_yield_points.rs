macro_rules! deps {
    () => {
        HighlightedRange!();
        WalkExpandedExprCtx!();
        HighlightMap!();
    };
}

macro_rules! highlight_yield_points {
    () => {
        deps!();
        pub (crate) fn highlight_yield_points (sema : & Semantics < '_ , RootDatabase > , token : SyntaxToken ,) -> FxHashMap < EditionedFileId , Vec < HighlightedRange > > { fn hl (sema : & Semantics < '_ , RootDatabase > , async_token : Option < SyntaxToken > , body : Option < ast :: Expr > ,) -> Option < HighlightMap > { let mut highlights : FxHashMap < EditionedFileId , FxHashSet < _ > > = FxHashMap :: default () ; let mut push_to_highlights = | file_id , range | { if let Some (FileRange { file_id , range }) = original_frange (sema . db , file_id , range) { let hrange = HighlightedRange { category : ReferenceCategory :: empty () , range } ; highlights . entry (file_id) . or_default () . insert (hrange) ; } } ; let async_token = async_token ? ; let async_tok_file_id = sema . hir_file_for (& async_token . parent () ?) ; push_to_highlights (async_tok_file_id , Some (async_token . text_range ())) ; let Some (body) = body else { return Some (highlights) ; } ; WalkExpandedExprCtx :: new (sema) . walk (& body , & mut | _ , expr | { let file_id = sema . hir_file_for (expr . syntax ()) ; let text_range = match expr { ast :: Expr :: AwaitExpr (expr) => expr . await_token () , ast :: Expr :: ReturnExpr (expr) => expr . return_token () , _ => None , } . map (| it | it . text_range ()) ; push_to_highlights (file_id , text_range) ; }) ; Some (highlights) } let mut res = FxHashMap :: default () ; for anc in goto_definition :: find_fn_or_blocks (sema , & token) { let new_map = match_ast ! { match anc { ast :: Fn (fn_) => hl (sema , fn_ . async_token () , fn_ . body () . map (ast :: Expr :: BlockExpr)) , ast :: BlockExpr (block_expr) => { let Some (async_token) = block_expr . async_token () else { continue ; } ; if async_token == token { let exit_points = hl_exit_points (sema , Some (async_token . clone ()) , block_expr . clone () . into () ,) ; merge_map (& mut res , exit_points) ; } hl (sema , Some (async_token) , Some (block_expr . into ())) } , ast :: ClosureExpr (closure) => hl (sema , closure . async_token () , closure . body ()) , _ => continue , } } ; merge_map (& mut res , new_map) ; } res . into_iter () . map (| (file_id , ranges) | (file_id , ranges . into_iter () . collect ())) . collect () }
    };
}

highlight_yield_points!()