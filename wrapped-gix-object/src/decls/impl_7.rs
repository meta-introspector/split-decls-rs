macro_rules! deps {
    () => {
        TrailerRef!();
        Trailers!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < 'a > Iterator for Trailers < 'a > { type Item = TrailerRef < 'a > ; fn next (& mut self) -> Option < Self :: Item > { if self . cursor . is_empty () { return None ; } for mut line in self . cursor . lines_with_terminator () { self . cursor = & self . cursor [line . len () ..] ; if let Some (trailer) = terminated (parse_single_line_trailer :: < () > , eof) . parse_next (& mut line) . ok () . map (| (token , value) | TrailerRef { token : token . trim () . as_bstr () , value : value . trim () . as_bstr () , }) { return Some (trailer) ; } } None } }
    };
}

impl_7!()