macro_rules! deps {
    () => {
        HighlightMap!();
        HighlightedRange!();
        WalkExpandedExprCtx!();
    };
}

macro_rules! hl_exit_points {
    () => {
        deps!();
        fn hl_exit_points (sema : & Semantics < '_ , RootDatabase > , def_token : Option < SyntaxToken > , body : ast :: Expr ,) -> Option < HighlightMap > { let mut highlights : FxHashMap < EditionedFileId , FxHashSet < _ > > = FxHashMap :: default () ; let mut push_to_highlights = | file_id , range | { if let Some (FileRange { file_id , range }) = original_frange (sema . db , file_id , range) { let hrange = HighlightedRange { category : ReferenceCategory :: empty () , range } ; highlights . entry (file_id) . or_default () . insert (hrange) ; } } ; if let Some (tok) = def_token { let file_id = sema . hir_file_for (& tok . parent () ?) ; let range = Some (tok . text_range ()) ; push_to_highlights (file_id , range) ; } WalkExpandedExprCtx :: new (sema) . walk (& body , & mut | _ , expr | { let file_id = sema . hir_file_for (expr . syntax ()) ; let range = match & expr { ast :: Expr :: TryExpr (try_) => try_ . question_mark_token () . map (| token | token . text_range ()) , ast :: Expr :: MethodCallExpr (_) | ast :: Expr :: CallExpr (_) | ast :: Expr :: MacroExpr (_) if sema . type_of_expr (& expr) . is_some_and (| ty | ty . original . is_never ()) => { Some (expr . syntax () . text_range ()) } _ => None , } ; push_to_highlights (file_id , range) ; }) ; WalkExpandedExprCtx :: new (sema) . with_check_ctx (& WalkExpandedExprCtx :: is_async_const_block_or_closure) . walk (& body , & mut | _ , expr | { let file_id = sema . hir_file_for (expr . syntax ()) ; let range = match & expr { ast :: Expr :: ReturnExpr (expr) => expr . return_token () . map (| token | token . text_range ()) , _ => None , } ; push_to_highlights (file_id , range) ; }) ; let tail = match body { ast :: Expr :: BlockExpr (b) => b . tail_expr () , e => Some (e) , } ; if let Some (tail) = tail { for_each_tail_expr (& tail , & mut | tail | { let file_id = sema . hir_file_for (tail . syntax ()) ; let range = match tail { ast :: Expr :: BreakExpr (b) => b . break_token () . map_or_else (| | tail . syntax () . text_range () , | tok | tok . text_range ()) , _ => tail . syntax () . text_range () , } ; push_to_highlights (file_id , Some (range)) ; }) ; } Some (highlights) }
    };
}

hl_exit_points!()