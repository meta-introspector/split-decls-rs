macro_rules! deps {
    () => {
        PatternID!();
    };
}

macro_rules! read_pattern_id_unchecked {
    () => {
        deps!();
        # [doc = " Reads a pattern ID from the given slice. If the slice has insufficient"] # [doc = " length, then this panics. Otherwise, the deserialized integer is assumed"] # [doc = " to be a valid pattern ID."] # [doc = ""] # [doc = " This also returns the number of bytes read."] pub (crate) fn read_pattern_id_unchecked (slice : & [u8]) -> (PatternID , usize) { let pid = PatternID :: from_ne_bytes_unchecked (slice [.. PatternID :: SIZE] . try_into () . unwrap () ,) ; (pid , PatternID :: SIZE) }
    };
}

read_pattern_id_unchecked!()