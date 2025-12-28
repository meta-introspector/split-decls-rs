macro_rules! deps {
    () => {
        Hunk!();
    };
}

macro_rules! truncate_hunks_from_from_back {
    () => {
        deps!();
        fn truncate_hunks_from_from_back (hunks : & mut Vec < Hunk > , remove_trailing_hunks_from_idx : Option < usize > , hunk_token_equal_from : Option < u32 > , mut out_hunks : Option < & mut Vec < Hunk > > ,) { let Some (mut remove_trailing_hunks_from_idx) = remove_trailing_hunks_from_idx else { assert ! (hunk_token_equal_from . is_none ()) ; return ; } ; let hunk = & mut hunks [remove_trailing_hunks_from_idx] ; let range = range_by_side (hunk) ; if let Some (hunk_token_equal_from) = hunk_token_equal_from { let orig_end = range . end ; let new_end = hunk_token_equal_from ; range . end = new_end ; if Range :: < u32 > :: is_empty (range) { range . end = orig_end ; } else if let Some (out) = out_hunks . as_deref_mut () { remove_trailing_hunks_from_idx += 1 ; let mut removed_hunk = hunk . clone () ; let new_range = range_by_side (& mut removed_hunk) ; new_range . start = new_end ; new_range . end = orig_end ; out . push (removed_hunk) ; } else { remove_trailing_hunks_from_idx += 1 ; } } if let Some (out) = out_hunks { out . extend_from_slice (& hunks [remove_trailing_hunks_from_idx ..]) ; } hunks . truncate (remove_trailing_hunks_from_idx) ; }
    };
}

truncate_hunks_from_from_back!();