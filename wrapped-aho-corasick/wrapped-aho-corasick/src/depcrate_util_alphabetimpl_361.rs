// Generated macro for impl_361 (impl)
macro_rules! Depcrate_util_alphabetimpl_361 {
() => {
// Module: crate::util::alphabet
// Provides: {"impl_361"}
// Dependencies: {}
impl ByteClassSet { # [doc = " Create a new set of byte classes where all bytes are part of the same"] # [doc = " equivalence class."] pub (crate) fn empty () -> Self { ByteClassSet (ByteSet :: empty ()) } # [doc = " Indicate the the range of byte given (inclusive) can discriminate a"] # [doc = " match between it and all other bytes outside of the range."] pub (crate) fn set_range (& mut self , start : u8 , end : u8) { debug_assert ! (start <= end) ; if start > 0 { self . 0 . add (start - 1) ; } self . 0 . add (end) ; } # [doc = " Convert this boolean set to a map that maps all byte values to their"] # [doc = " corresponding equivalence class. The last mapping indicates the largest"] # [doc = " equivalence class identifier (which is never bigger than 255)."] pub (crate) fn byte_classes (& self) -> ByteClasses { let mut classes = ByteClasses :: empty () ; let mut class = 0u8 ; let mut b = 0u8 ; loop { classes . set (b , class) ; if b == 255 { break ; } if self . 0 . contains (b) { class = class . checked_add (1) . unwrap () ; } b = b . checked_add (1) . unwrap () ; } classes } }
};
}
