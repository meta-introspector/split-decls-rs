macro_rules! read_literal {
    () => {
        fn read_literal (input : & str) -> (& str , & str) { let mut start = None ; let mut end = 0 ; for (pos , c) in input . bytes () . enumerate () { if start . is_none () { if c != b' ' && c != b',' { start = Some (pos) ; } } else if c == b' ' || c == b',' || c == b'}' { break ; } end += 1 ; } let Some (start) = start else { panic ! () ; } ; (& input [start .. end] , & input [end ..]) }
    };
}

read_literal!()