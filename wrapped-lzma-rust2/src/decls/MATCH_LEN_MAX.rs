macro_rules! MATCH_LEN_MAX {
    () => {
        const MATCH_LEN_MAX : usize = MATCH_LEN_MIN + LOW_SYMBOLS + MID_SYMBOLS + HIGH_SYMBOLS - 1 ;
    };
}

MATCH_LEN_MAX!();