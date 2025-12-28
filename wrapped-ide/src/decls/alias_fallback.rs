macro_rules! alias_fallback {
    () => {
        fn alias_fallback (syntax : & SyntaxNode , FilePosition { file_id , offset } : FilePosition , new_name : & str ,) -> Option < SourceChange > { let use_tree = syntax . token_at_offset (offset) . flat_map (| syntax | syntax . parent_ancestors ()) . find_map (ast :: UseTree :: cast) ? ; let last_path_segment = use_tree . path () ? . segments () . last () ? . name_ref () ? ; if ! last_path_segment . syntax () . text_range () . contains_inclusive (offset) { return None ; } ; let mut builder = SourceChangeBuilder :: new (file_id) ; match use_tree . rename () { Some (rename) => { let offset = rename . syntax () . text_range () ; builder . replace (offset , format ! ("as {new_name}")) ; } None => { let offset = use_tree . syntax () . text_range () . end () ; builder . insert (offset , format ! (" as {new_name}")) ; } } Some (builder . finish ()) }
    };
}

alias_fallback!()