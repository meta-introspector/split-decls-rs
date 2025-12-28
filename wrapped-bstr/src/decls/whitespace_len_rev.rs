macro_rules! whitespace_len_rev {
    () => {
        # [doc = " Return the last position of a non-whitespace character."] pub fn whitespace_len_rev (slice : & [u8]) -> usize { let input = Input :: new (slice) . anchored (Anchored :: Yes) ; WHITESPACE_ANCHORED_REV . try_search_rev (& input) . unwrap () . map_or (slice . len () , | hm | hm . offset ()) }
    };
}

whitespace_len_rev!();