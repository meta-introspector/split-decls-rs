macro_rules! docs_from_attrs {
    () => {
        pub fn docs_from_attrs (attrs : & hir :: Attrs) -> Option < String > { let docs = attrs . by_key (sym :: doc) . attrs () . filter_map (| attr | attr . string_value_unescape ()) ; let indent = doc_indent (attrs) ; let mut buf = String :: new () ; for doc in docs { if ! doc . is_empty () { let lines = doc . lines () . map (| line | { line . char_indices () . nth (indent) . map_or (line , | (offset , _) | & line [offset ..]) }) ; buf . extend (Itertools :: intersperse (lines , "\n")) ; } buf . push ('\n') ; } buf . pop () ; if buf . is_empty () { None } else { Some (buf) } }
    };
}

docs_from_attrs!()