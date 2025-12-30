// Generated macro for gb18030_range_decode (function)
macro_rules! Depcrate_datagb18030_range_decode {
() => {
// Module: crate::data
// Provides: {"gb18030_range_decode"}
// Dependencies: {}
# [inline (always)] pub fn gb18030_range_decode (pointer : u16) -> u16 { map_with_ranges (& GB18030_RANGE_POINTERS [..] , & GB18030_RANGE_OFFSETS [..] , pointer ,) }
};
}
