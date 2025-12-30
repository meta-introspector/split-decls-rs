// Generated macro for gbk_other_decode (function)
macro_rules! Depcrate_datagbk_other_decode {
() => {
// Module: crate::data
// Provides: {"gbk_other_decode"}
// Dependencies: {}
# [inline (always)] pub fn gbk_other_decode (pointer : u16) -> u16 { map_with_ranges (& GBK_OTHER_POINTERS [.. GBK_OTHER_POINTERS . len () - 1] , & GBK_OTHER_UNSORTED_OFFSETS [..] , pointer ,) }
};
}
