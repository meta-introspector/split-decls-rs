// Generated macro for Take (struct)
macro_rules! Depcrate_buf_takeTake {
() => {
// Module: crate::buf::take
// Provides: {"Take"}
// Dependencies: {}
# [doc = " A `Buf` adapter which limits the bytes read from an underlying buffer."] # [doc = ""] # [doc = " This struct is generally created by calling `take()` on `Buf`. See"] # [doc = " documentation of [`take()`](Buf::take) for more details."] # [derive (Debug)] pub struct Take < T > { inner : T , limit : usize , }
};
}
