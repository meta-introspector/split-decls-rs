macro_rules! deps {
    () => {
        LineIndexDatabase!();
    };
}

macro_rules! line_index {
    () => {
        deps!();
        fn line_index (db : & dyn LineIndexDatabase , file_id : FileId) -> Arc < LineIndex > { let text = db . file_text (file_id) . text (db) ; Arc :: new (LineIndex :: new (text)) }
    };
}

line_index!()