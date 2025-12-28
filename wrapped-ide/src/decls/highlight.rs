macro_rules! deps {
    () => {
        HighlightConfig!();
        HlRange!();
    };
}

macro_rules! highlight {
    () => {
        deps!();
        pub (crate) fn highlight (db : & RootDatabase , config : & HighlightConfig < '_ > , file_id : FileId , range_to_highlight : Option < TextRange > ,) -> Vec < HlRange > { let _p = tracing :: info_span ! ("highlight") . entered () ; let sema = Semantics :: new (db) ; let file_id = sema . attach_first_edition (file_id) . unwrap_or_else (| | EditionedFileId :: current_edition (db , file_id)) ; let (root , range_to_highlight) = { let file = sema . parse (file_id) ; let source_file = file . syntax () ; match range_to_highlight { Some (range) => { let node = match source_file . covering_element (range) { NodeOrToken :: Node (it) => it , NodeOrToken :: Token (it) => it . parent () . unwrap_or_else (| | source_file . clone ()) , } ; (node , range) } None => (source_file . clone () , source_file . text_range ()) , } } ; let mut hl = highlights :: Highlights :: new (root . text_range ()) ; let krate = sema . scope (& root) . map (| it | it . krate ()) ; traverse (& mut hl , & sema , config , InRealFile :: new (file_id , & root) , krate , range_to_highlight) ; hl . to_vec () }
    };
}

highlight!();