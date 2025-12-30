// Generated macro for gb2312_other_decode (function)
macro_rules! Depcrate_datagb2312_other_decode {
() => {
// Module: crate::data
// Provides: {"gb2312_other_decode"}
// Dependencies: {}
# [inline (always)] pub fn gb2312_other_decode (pointer : u16) -> u16 { map_with_ranges (& GB2312_OTHER_POINTERS [.. GB2312_OTHER_POINTERS . len () - 1] , & GB2312_OTHER_UNSORTED_OFFSETS [..] , pointer ,) }
};
}
