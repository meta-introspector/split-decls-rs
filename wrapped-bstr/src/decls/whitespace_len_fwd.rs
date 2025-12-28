macro_rules! whitespace_len_fwd {
    () => {
        # [doc = " Return the first position of a non-whitespace character."] pub fn whitespace_len_fwd (slice : & [u8]) -> usize { let input = Input :: new (slice) . anchored (Anchored :: Yes) ; WHITESPACE_ANCHORED_FWD . try_search_fwd (& input) . unwrap () . map_or (0 , | hm | hm . offset ()) }
    };
}

whitespace_len_fwd!();