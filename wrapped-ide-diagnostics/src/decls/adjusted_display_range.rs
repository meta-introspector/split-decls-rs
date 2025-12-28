macro_rules! deps {
    () => {
        DiagnosticsContext!();
    };
}

macro_rules! adjusted_display_range {
    () => {
        deps!();
        fn adjusted_display_range < N : AstNode > (ctx : & DiagnosticsContext < '_ > , diag_ptr : InFile < AstPtr < N > > , adj : & dyn Fn (N) -> Option < TextRange > ,) -> FileRange { let source_file = ctx . sema . parse_or_expand (diag_ptr . file_id) ; let node = diag_ptr . value . to_node (& source_file) ; let hir :: FileRange { file_id , range } = diag_ptr . with_value (adj (node) . unwrap_or_else (| | diag_ptr . value . text_range ())) . original_node_file_range_rooted (ctx . sema . db) ; ide_db :: FileRange { file_id : file_id . file_id (ctx . sema . db) , range } }
    };
}

adjusted_display_range!()