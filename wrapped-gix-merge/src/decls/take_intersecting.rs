macro_rules! deps {
    () => {
        Hunk!();
    };
}

macro_rules! take_intersecting {
    () => {
        deps!();
        # [doc = " Find all hunks in `iter` which aren't from the same side as `hunk` and intersect with it."] # [doc = " Also put `hunk` into `input` so it's the first item, and possibly put more hunks of the side of `hunk` so"] # [doc = " `iter` doesn't have any overlapping hunks left."] # [doc = " Return `Some` if `intersecting` is non-empty after the operation, indicating overlapping hunks were found."] pub fn take_intersecting (iter : & mut Peekable < impl Iterator < Item = Hunk > > , input : & mut Vec < Hunk > , intersecting : & mut Vec < Hunk > ,) -> Option < () > { input . clear () ; input . push (iter . next () ?) ; intersecting . clear () ; fn left_overlaps_right (left : & Hunk , right : & Hunk) -> bool { left . side != right . side && (right . before . contains (& left . before . start) || (right . before . is_empty () && right . before . start == left . before . start)) } loop { let hunk = input . last () . expect ("just pushed") ; while iter . peek () . filter (| b_hunk | left_overlaps_right (b_hunk , hunk)) . is_some () { intersecting . extend (iter . next ()) ; } let mut found_more_intersections = false ; while intersecting . last_mut () . zip (iter . peek_mut ()) . filter (| (last_intersecting , candidate) | left_overlaps_right (candidate , last_intersecting)) . is_some () { input . extend (iter . next ()) ; found_more_intersections = true ; } if ! found_more_intersections { break ; } } Some (()) }
    };
}

take_intersecting!();