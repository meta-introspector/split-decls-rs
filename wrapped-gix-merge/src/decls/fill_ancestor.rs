macro_rules! deps {
    () => {
        Hunk!();
    };
}

macro_rules! fill_ancestor {
    () => {
        deps!();
        # [doc = " Look at all hunks in `in_out` and fill in the ancestor in the range of `ancestor_range`."] # [doc = " This is all based on knowing the ranges are sequences of tokens."] pub fn fill_ancestor (Range { start , end } : & Range < u32 > , in_out : & mut Vec < Hunk >) { fn is_nonzero (num : & u32) -> bool { * num > 0 } if in_out . is_empty () { return ; } let first = & in_out [0] ; let mut first_idx = 0 ; if let Some (lines_to_add) = first . before . start . checked_sub (* start) . filter (is_nonzero) { in_out . insert (0 , ancestor_hunk (* start , lines_to_add)) ; first_idx += 1 ; } let mut added_hunks = false ; for (idx , next_idx) in (first_idx .. in_out . len ()) . map (| idx | (idx , idx + 1)) { let Some (next_hunk) = in_out . get (next_idx) else { break } ; let hunk = & in_out [idx] ; if let Some (lines_to_add) = next_hunk . before . start . checked_sub (hunk . before . end) . filter (is_nonzero) { in_out . push (ancestor_hunk (hunk . before . end , lines_to_add)) ; added_hunks = true ; } } let in_out_len = in_out . len () ; if added_hunks { in_out [first_idx .. in_out_len] . sort_by_key (| hunk | hunk . before . start) ; } let last = & in_out [in_out_len - 1] ; if let Some (lines_to_add) = end . checked_sub (last . before . end) . filter (is_nonzero) { in_out . push (ancestor_hunk (last . before . end , lines_to_add)) ; } }
    };
}

fill_ancestor!()