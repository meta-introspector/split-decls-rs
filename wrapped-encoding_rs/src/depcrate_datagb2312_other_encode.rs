// Generated macro for gb2312_other_encode (function)
macro_rules! Depcrate_datagb2312_other_encode {
() => {
// Module: crate::data
// Provides: {"gb2312_other_encode"}
// Dependencies: {}
# [inline (always)] pub fn gb2312_other_encode (bmp : u16) -> Option < u16 > { map_with_unsorted_ranges (& GB2312_OTHER_UNSORTED_OFFSETS [..] , & GB2312_OTHER_POINTERS [..] , bmp ,) }
};
}
