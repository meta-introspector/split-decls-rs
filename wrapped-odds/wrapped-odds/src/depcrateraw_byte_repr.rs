// Generated macro for raw_byte_repr (function)
macro_rules! Depcrateraw_byte_repr {
() => {
// Module: crate
// Provides: {"raw_byte_repr"}
// Dependencies: {}
# [doc = " Safe to use with any wholly initialized memory `ptr`"] # [inline] pub unsafe fn raw_byte_repr < T : ? Sized > (ptr : & T) -> & [u8] { std :: slice :: from_raw_parts (ptr as * const _ as * const u8 , mem :: size_of_val (ptr)) }
};
}
