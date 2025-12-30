// Generated macro for skip_before_separator (function)
macro_rules! Depcrate_parserskip_before_separator {
() => {
// Module: crate::parser
// Provides: {"skip_before_separator"}
// Dependencies: {}
const fn skip_before_separator (slice : & [u8]) -> & [u8] { let mut end = 0 ; # [expect (clippy :: indexing_slicing)] while end < slice . len () && ! matches ! (slice [end] , b'-') { end += 1 ; } unsafe { slice . split_at_unchecked (end) . 0 } }
};
}
