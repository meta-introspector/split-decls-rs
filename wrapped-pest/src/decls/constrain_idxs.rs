macro_rules! constrain_idxs {
    () => {
        # [doc = " Helper function used only in case stack operations (PUSH/POP) are used in grammar."] fn constrain_idxs (start : i32 , end : Option < i32 > , len : usize) -> Option < Range < usize > > { let start_norm = normalize_index (start , len) ? ; let end_norm = end . map_or (Some (len) , | e | normalize_index (e , len)) ? ; Some (start_norm .. end_norm) }
    };
}

constrain_idxs!();