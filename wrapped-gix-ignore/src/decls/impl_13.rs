macro_rules! deps {
    () => {
        Kind!();
        Lines!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Iterator for Lines < '_ > { type Item = (gix_glob :: Pattern , usize , crate :: Kind) ; fn next (& mut self) -> Option < Self :: Item > { for mut line in self . lines . by_ref () { self . line_no += 1 ; let first = match line . first () . copied () { Some (b'#') | None => continue , Some (c) => c , } ; let (kind , can_negate) = if self . support_precious && first == b'$' { line = & line [1 ..] ; (crate :: Kind :: Precious , false) } else { let second = line . get (1) ; if first == b'!' && second == Some (& b'$') { gix_trace :: error ! ("Line {} starts with !$ which is not allowed ('{}')" , self . line_no , line . as_bstr ()) ; continue ; } if first == b'\\' && second == Some (& b'$') { line = & line [1 ..] ; } (crate :: Kind :: Expendable , true) } ; line = truncate_non_escaped_trailing_spaces (line) ; let res = if can_negate { gix_glob :: Pattern :: from_bytes (line) } else { gix_glob :: Pattern :: from_bytes_without_negation (line) } ; match res { None => continue , Some (pattern) => return Some ((pattern , self . line_no , kind)) , } } None } }
    };
}

impl_13!();