macro_rules! decode_word {
    () => {
        fn decode_word (bs : & [u8]) -> (& str , usize) { if bs . is_empty () { ("" , 0) } else if let Some (hm) = { let input = Input :: new (bs) . anchored (Anchored :: Yes) ; WORD_BREAK_FWD . try_search_fwd (& input) . unwrap () } { let word = unsafe { bs [.. hm . offset ()] . to_str_unchecked () } ; (word , word . len ()) } else { const INVALID : & str = "\u{FFFD}" ; let (_ , size) = utf8 :: decode_lossy (bs) ; (INVALID , size) } }
    };
}

decode_word!()