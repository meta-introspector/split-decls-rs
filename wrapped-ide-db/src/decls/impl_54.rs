macro_rules! deps {
    () => {
        DocsRangeMap!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl DocsRangeMap { # [doc = " Maps a [`TextRange`] relative to the documentation string back to its AST range"] pub fn map (& self , range : TextRange) -> Option < (InFile < TextRange > , AttrId) > { let found = self . mapping . binary_search_by (| (probe , ..) | probe . ordering (range)) . ok () ? ; let (line_docs_range , idx , original_line_src_range) = self . mapping [found] ; if ! line_docs_range . contains_range (range) { return None ; } let relative_range = range - line_docs_range . start () ; let InFile { file_id , value : source } = self . source_map . source_of_id (idx) ; match source { Either :: Left (attr) => { let string = get_doc_string_in_attr (attr) ? ; let text_range = string . open_quote_text_range () ? ; let range = TextRange :: at (text_range . end () + original_line_src_range . start () + relative_range . start () , string . syntax () . text_range () . len () . min (range . len ()) ,) ; Some ((InFile { file_id , value : range } , idx)) } Either :: Right (comment) => { let text_range = comment . syntax () . text_range () ; let range = TextRange :: at (text_range . start () + TextSize :: try_from (comment . prefix () . len ()) . ok () ? + original_line_src_range . start () + relative_range . start () , text_range . len () . min (range . len ()) ,) ; Some ((InFile { file_id , value : range } , idx)) } } } pub fn shift_docstring_line_range (self , offset : TextSize) -> DocsRangeMap { let mapping = self . mapping . into_iter () . map (| (buf_offset , id , base_offset) | { let buf_offset = buf_offset . checked_add (offset) . unwrap () ; (buf_offset , id , base_offset) }) . collect_vec () ; DocsRangeMap { source_map : self . source_map , mapping } } }
    };
}

impl_54!()