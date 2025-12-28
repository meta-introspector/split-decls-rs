macro_rules! name_hygiene {
    () => {
        pub (crate) fn name_hygiene (db : & dyn HirDatabase , name : InFile < & SyntaxNode >) -> HygieneId { let Some (macro_file) = name . file_id . macro_file () else { return HygieneId :: ROOT ; } ; let span_map = db . expansion_span_map (macro_file) ; let ctx = span_map . span_at (name . value . text_range () . start ()) . ctx ; HygieneId :: new (ctx . opaque_and_semitransparent (db)) }
    };
}

name_hygiene!()