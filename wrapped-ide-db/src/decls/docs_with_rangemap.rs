macro_rules! deps {
    () => {
        DocsRangeMap!();
        Documentation!();
    };
}

macro_rules! docs_with_rangemap {
    () => {
        deps!();
        pub fn docs_with_rangemap (db : & dyn DefDatabase , attrs : & AttrsWithOwner ,) -> Option < (Documentation , DocsRangeMap) > { let docs = attrs . by_key (sym :: doc) . attrs () . filter_map (| attr | attr . string_value_unescape () . map (| s | (s , attr . id))) ; let indent = doc_indent (attrs) ; let mut buf = String :: new () ; let mut mapping = Vec :: new () ; for (doc , idx) in docs { if ! doc . is_empty () { let mut base_offset = 0 ; for raw_line in doc . split ('\n') { let line = raw_line . trim_end () ; let line_len = line . len () ; let (offset , line) = match line . char_indices () . nth (indent) { Some ((offset , _)) => (offset , & line [offset ..]) , None => (0 , line) , } ; let buf_offset = buf . len () ; buf . push_str (line) ; mapping . push ((TextRange :: new (buf_offset . try_into () . ok () ? , buf . len () . try_into () . ok () ?) , idx , TextRange :: at ((base_offset + offset) . try_into () . ok () ? , line_len . try_into () . ok () ? ,) ,)) ; buf . push ('\n') ; base_offset += raw_line . len () + 1 ; } } else { buf . push ('\n') ; } } buf . pop () ; if buf . is_empty () { None } else { Some ((Documentation (buf) , DocsRangeMap { mapping , source_map : attrs . source_map (db) })) } }
    };
}

docs_with_rangemap!()