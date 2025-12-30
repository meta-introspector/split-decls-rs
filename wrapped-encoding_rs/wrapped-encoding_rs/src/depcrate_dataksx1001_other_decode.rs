// Generated macro for ksx1001_other_decode (function)
macro_rules! Depcrate_dataksx1001_other_decode {
() => {
// Module: crate::data
// Provides: {"ksx1001_other_decode"}
// Dependencies: {}
# [inline (always)] pub fn ksx1001_other_decode (pointer : u16) -> u16 { map_with_ranges (& KSX1001_OTHER_POINTERS [.. KSX1001_OTHER_POINTERS . len () - 1] , & KSX1001_OTHER_UNSORTED_OFFSETS [..] , pointer ,) }
};
}
