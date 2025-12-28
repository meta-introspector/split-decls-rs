macro_rules! normalize_index {
    () => {
        # [doc = " `constrain_idxs` helper function."] # [doc = " Normalizes the index using its sequence’s length."] # [doc = " Returns `None` if the normalized index is OOB."] fn normalize_index (i : i32 , len : usize) -> Option < usize > { if i > len as i32 { None } else if i >= 0 { Some (i as usize) } else { let real_i = len as i32 + i ; if real_i >= 0 { Some (real_i as usize) } else { None } } }
    };
}

normalize_index!()