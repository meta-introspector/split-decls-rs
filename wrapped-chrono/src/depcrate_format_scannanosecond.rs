// Generated macro for nanosecond (function)
macro_rules! Depcrate_format_scannanosecond {
() => {
// Module: crate::format::scan
// Provides: {"nanosecond"}
// Dependencies: {}
# [doc = " Tries to consume at least one digits as a fractional second."] # [doc = " Returns the number of whole nanoseconds (0--999,999,999)."] pub (super) fn nanosecond (s : & str) -> ParseResult < (& str , u32) > { let origlen = s . len () ; let (s , v) = number (s , 1 , 9) ? ; let v = u32 :: try_from (v) . expect ("999,999,999 should fit u32") ; let consumed = origlen - s . len () ; const SCALE : [u32 ; 10] = [0 , 100_000_000 , 10_000_000 , 1_000_000 , 100_000 , 10_000 , 1_000 , 100 , 10 , 1] ; let v = v . checked_mul (SCALE [consumed]) . ok_or (OUT_OF_RANGE) ? ; let s = s . trim_start_matches (| c : char | c . is_ascii_digit ()) ; Ok ((s , v)) }
};
}
