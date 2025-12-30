// Generated macro for blend (function)
macro_rules! Depcrate_bitmap_pixel_pixel_formatblend {
() => {
// Module: crate::bitmap_pixel::pixel_format
// Provides: {"blend"}
// Dependencies: {}
# [inline (always)] pub (super) fn blend (prev : & mut u8 , new : u8 , a : u64) { if new > * prev { * prev += (u64 :: from (new - * prev) * a / 256) as u8 } else { * prev -= (u64 :: from (* prev - new) * a / 256) as u8 } }
};
}
