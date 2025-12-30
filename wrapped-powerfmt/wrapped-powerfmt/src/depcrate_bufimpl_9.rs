// Generated macro for impl_9 (impl)
macro_rules! Depcrate_bufimpl_9 {
() => {
// Module: crate::buf
// Provides: {"impl_9"}
// Dependencies: {}
impl < const SIZE : usize > WriteBuffer < SIZE > { # [doc = " Creates an empty buffer."] pub const fn new () -> Self { Self { buf : maybe_uninit_uninit_array :: < _ , SIZE > () , len : 0 , } } # [doc = " Obtain the contents of the buffer as a string."] pub fn as_str (& self) -> & str { self } # [doc = " Determine how many bytes are remaining in the buffer."] pub const fn remaining_capacity (& self) -> usize { SIZE - self . len } }
};
}
