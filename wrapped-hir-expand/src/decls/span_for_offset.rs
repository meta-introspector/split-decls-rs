macro_rules! span_for_offset {
    () => {
        # [doc = " Looks up the span at the given offset."] pub fn span_for_offset (db : & dyn ExpandDatabase , exp_map : & ExpansionSpanMap , offset : TextSize ,) -> (FileRange , SyntaxContext) { let span = exp_map . span_at (offset) ; let file_id = EditionedFileId :: from_span (db , span . anchor . file_id) ; let anchor_offset = db . ast_id_map (file_id . into ()) . get_erased (span . anchor . ast_id) . text_range () . start () ; (FileRange { file_id , range : span . range + anchor_offset } , span . ctx) }
    };
}

span_for_offset!()