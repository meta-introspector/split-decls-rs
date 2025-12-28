macro_rules! find_char_midpoint {
    () => {
        # [doc = " Find the index of a character boundary near the midpoint."] # [inline] fn find_char_midpoint (chars : & str) -> usize { let mid = chars . len () / 2 ; let (left , right) = chars . as_bytes () . split_at (mid) ; match right . iter () . copied () . position (is_char_boundary) { Some (i) => mid + i , None => left . iter () . copied () . rposition (is_char_boundary) . unwrap_or (0) , } }
    };
}

find_char_midpoint!()