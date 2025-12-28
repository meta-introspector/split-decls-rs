macro_rules! deps {
    () => {
        HighlightedRange!();
    };
}

macro_rules! highlight_closure_captures {
    () => {
        deps!();
        fn highlight_closure_captures (sema : & Semantics < '_ , RootDatabase > , token : SyntaxToken , file_id : EditionedFileId ,) -> Option < Vec < HighlightedRange > > { let closure = token . parent_ancestors () . take (2) . find_map (ast :: ClosureExpr :: cast) ? ; let search_range = closure . body () ? . syntax () . text_range () ; let ty = & sema . type_of_expr (& closure . into ()) ? . original ; let c = ty . as_closure () ? ; Some (c . captured_items (sema . db) . into_iter () . map (| capture | capture . local ()) . flat_map (| local | { let usages = Definition :: Local (local) . usages (sema) . in_scope (& SearchScope :: file_range (FileRange { file_id , range : search_range })) . include_self_refs () . all () . references . remove (& file_id) . into_iter () . flatten () . map (| FileReference { category , range , .. } | HighlightedRange { range , category , }) ; let category = if local . is_mut (sema . db) { ReferenceCategory :: WRITE } else { ReferenceCategory :: empty () } ; local . sources (sema . db) . into_iter () . flat_map (| x | x . to_nav (sema . db)) . filter (| decl | decl . file_id == file_id . file_id (sema . db)) . filter_map (| decl | decl . focus_range) . map (move | range | HighlightedRange { range , category }) . chain (usages) }) . collect () ,) }
    };
}

highlight_closure_captures!()