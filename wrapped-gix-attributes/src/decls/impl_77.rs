macro_rules! deps {
    () => {
        Iter!();
        Kind!();
        Error!();
        Lines!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < 'a > Iterator for Lines < 'a > { type Item = Result < (Kind , Iter < 'a > , usize) , Error > ; fn next (& mut self) -> Option < Self :: Item > { fn skip_blanks (line : & BStr) -> & BStr { line . find_not_byteset (BLANKS) . map_or (line , | pos | & line [pos ..]) } for line in self . lines . by_ref () { self . line_no += 1 ; let line = skip_blanks (line . into ()) ; if line . first () == Some (& b'#') { continue ; } match parse_line (line , self . line_no) { None => continue , Some (res) => return Some (res) , } } None } }
    };
}

impl_77!()