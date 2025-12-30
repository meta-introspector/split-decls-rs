// Generated macro for impl_82 (impl)
macro_rules! Depcrate_decode_bytesimpl_82 {
() => {
// Module: crate::decode::bytes
// Provides: {"impl_82"}
// Dependencies: {}
impl < 'a > Bytes < 'a > { # [doc = " Wrap an existing bytes slice."] # [doc = ""] # [doc = " This sets the internal position to zero."] # [inline] # [must_use] pub const fn new (bytes : & 'a [u8]) -> Self { Bytes { bytes , current_position : 0 } } # [doc = " Get a reference to the remaining bytes in the buffer."] # [inline] # [must_use] pub const fn remaining_slice (& self) -> & 'a [u8] { self . bytes } # [doc = " Return the position of the input buffer."] # [doc = ""] # [doc = " This is not required for correctness, it only exists to help mimic"] # [doc = " [`Cursor::position`](std::io::Cursor::position)"] # [inline] # [must_use] pub const fn position (& self) -> u64 { self . current_position } }
};
}
