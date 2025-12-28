macro_rules! deps {
    () => {
        HighlightedRange!();
    };
}

macro_rules! highlight_unsafe_points {
    () => {
        deps!();
        pub (crate) fn highlight_unsafe_points (sema : & Semantics < '_ , RootDatabase > , token : SyntaxToken ,) -> FxHashMap < EditionedFileId , Vec < HighlightedRange > > { fn hl (sema : & Semantics < '_ , RootDatabase > , unsafe_token : & SyntaxToken , block_expr : Option < ast :: BlockExpr > ,) -> Option < FxHashMap < EditionedFileId , Vec < HighlightedRange > > > { let mut highlights : FxHashMap < EditionedFileId , Vec < _ > > = FxHashMap :: default () ; let mut push_to_highlights = | file_id , range | { if let Some (FileRange { file_id , range }) = original_frange (sema . db , file_id , range) { let hrange = HighlightedRange { category : ReferenceCategory :: empty () , range } ; highlights . entry (file_id) . or_default () . push (hrange) ; } } ; let unsafe_token_file_id = sema . hir_file_for (& unsafe_token . parent () ?) ; push_to_highlights (unsafe_token_file_id , Some (unsafe_token . text_range ())) ; if let Some (block) = block_expr { let unsafe_ops = sema . get_unsafe_ops_for_unsafe_block (block) ; for unsafe_op in unsafe_ops { push_to_highlights (unsafe_op . file_id , Some (unsafe_op . value . text_range ())) ; } } Some (highlights) } hl (sema , & token , token . parent () . and_then (ast :: BlockExpr :: cast)) . unwrap_or_default () }
    };
}

highlight_unsafe_points!()