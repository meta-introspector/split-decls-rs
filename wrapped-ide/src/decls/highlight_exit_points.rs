macro_rules! deps {
    () => {
        HighlightedRange!();
    };
}

macro_rules! highlight_exit_points {
    () => {
        deps!();
        pub (crate) fn highlight_exit_points (sema : & Semantics < '_ , RootDatabase > , token : SyntaxToken ,) -> FxHashMap < EditionedFileId , Vec < HighlightedRange > > { let mut res = FxHashMap :: default () ; for def in goto_definition :: find_fn_or_blocks (sema , & token) { let new_map = match_ast ! { match def { ast :: Fn (fn_) => fn_ . body () . and_then (| body | hl_exit_points (sema , fn_ . fn_token () , body . into ())) , ast :: ClosureExpr (closure) => { let pipe_tok = closure . param_list () . and_then (| p | p . pipe_token ()) ; closure . body () . and_then (| body | hl_exit_points (sema , pipe_tok , body)) } , ast :: BlockExpr (blk) => match blk . modifier () { Some (ast :: BlockModifier :: Async (t)) => hl_exit_points (sema , Some (t) , blk . into ()) , Some (ast :: BlockModifier :: Try (t)) if token . kind () != T ! [return] => { hl_exit_points (sema , Some (t) , blk . into ()) } , _ => continue , } , _ => continue , } } ; merge_map (& mut res , new_map) ; } res . into_iter () . map (| (file_id , ranges) | (file_id , ranges . into_iter () . collect ())) . collect () }
    };
}

highlight_exit_points!()