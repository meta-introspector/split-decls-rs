macro_rules! deps {
    () => {
        Hunk!();
    };
}

macro_rules! truncate_hunks_from_from_front {
    () => {
        deps!();
        fn truncate_hunks_from_from_front (hunks : & mut Vec < Hunk > , hunks_to_remove_until_idx : Option < usize > , hunk_token_equal_till : Option < u32 > , mut out_hunks : Option < & mut Vec < Hunk > > ,) { let Some (hunks_to_remove_until_idx) = hunks_to_remove_until_idx else { assert ! (hunk_token_equal_till . is_none ()) ; return ; } ; let mut last_index_to_remove = Some (hunks_to_remove_until_idx) ; let hunk = & mut hunks [hunks_to_remove_until_idx] ; let range = range_by_side (hunk) ; if let Some (hunk_token_equal_till) = hunk_token_equal_till { let orig_start = range . start ; let new_start = hunk_token_equal_till + 1 ; range . start = new_start ; if Range :: < u32 > :: is_empty (range) { range . start = orig_start ; } else if let Some (out) = out_hunks . as_deref_mut () { last_index_to_remove = hunks_to_remove_until_idx . checked_sub (1) ; let mut removed_hunk = hunk . clone () ; let new_range = range_by_side (& mut removed_hunk) ; new_range . start = orig_start ; new_range . end = new_start ; out . push (removed_hunk) ; } else { last_index_to_remove = hunks_to_remove_until_idx . checked_sub (1) ; } } if let Some (last_index_to_remove) = last_index_to_remove { let mut current_idx = 0 ; hunks . retain (| hunk | { if current_idx > last_index_to_remove { true } else { current_idx += 1 ; if let Some (out) = out_hunks . as_deref_mut () { out . push (hunk . clone ()) ; } false } }) ; } }
    };
}

truncate_hunks_from_from_front!()