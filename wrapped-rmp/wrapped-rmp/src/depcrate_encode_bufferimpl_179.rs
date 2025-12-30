// Generated macro for impl_179 (impl)
macro_rules! Depcrate_encode_bufferimpl_179 {
() => {
// Module: crate::encode::buffer
// Provides: {"impl_179"}
// Dependencies: {}
impl ByteBuf { # [doc = " Construct a new empty buffer"] # [inline] # [must_use] pub fn new () -> Self { Self { bytes : Vec :: new () } } # [doc = " Construct a new buffer with the specified capacity"] # [doc = ""] # [doc = " See [`Vec::with_capacity`] for details"] # [inline] # [must_use] pub fn with_capacity (capacity : usize) -> Self { Self { bytes : Vec :: with_capacity (capacity) , } } # [doc = " Unwrap the underlying buffer of this vector"] # [inline] # [must_use] pub fn into_vec (self) -> Vec < u8 > { self . bytes } # [doc = " Wrap the specified vector as a [`ByteBuf`]"] # [inline] # [must_use] pub fn from_vec (bytes : Vec < u8 >) -> Self { Self { bytes } } # [doc = " Get a reference to this type as a [Vec]"] # [inline] # [must_use] pub fn as_vec (& self) -> & Vec < u8 > { & self . bytes } # [doc = " Get a mutable reference to this type as a [Vec]"] # [inline] pub fn as_mut_vec (& mut self) -> & mut Vec < u8 > { & mut self . bytes } # [doc = " Get a reference to this type as a slice of bytes (`&[u8]`)"] # [inline] # [must_use] pub fn as_slice (& self) -> & [u8] { & self . bytes } }
};
}
