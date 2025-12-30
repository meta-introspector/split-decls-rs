// Generated macro for impl_22 (impl)
macro_rules! Depcrate_bufferimpl_22 {
() => {
// Module: crate::buffer
// Provides: {"impl_22"}
// Dependencies: {}
impl < const SIZE : usize > Buffer < SIZE > { # [doc = " Borrow the inner byte slice."] pub const fn as_bytes (& self) -> & [u8] { self . bytes . split_at (self . length as usize) . 0 } # [doc = " Get the length of the BER message."] pub const fn len (& self) -> usize { self . length as usize } # [doc = " Const comparison of two buffers."] pub const fn eq (& self , rhs : & Self) -> bool { if self . length != rhs . length { return false ; } let mut i = 0usize ; while i < self . len () { if self . bytes [i] != rhs . bytes [i] { return false ; } # [allow (clippy :: arithmetic_side_effects)] { i += 1 ; } } true } }
};
}
