// Generated macro for impl_264 (impl)
macro_rules! Depcrate_varintimpl_264 {
() => {
// Module: crate::varint
// Provides: {"impl_264"}
// Dependencies: {}
impl VarInt { # [doc = " The largest representable value"] pub const MAX : Self = Self ((1 << 62) - 1) ; # [doc = " The largest encoded value length"] pub const MAX_SIZE : usize = 8 ; # [doc = " Construct a `VarInt` infallibly"] pub const fn from_u32 (x : u32) -> Self { Self (x as u64) } # [doc = " Succeeds iff `x` < 2^62"] pub fn from_u64 (x : u64) -> Result < Self , VarIntBoundsExceeded > { if x < 2u64 . pow (62) { Ok (Self (x)) } else { Err (VarIntBoundsExceeded) } } # [doc = " Create a VarInt without ensuring it's in range"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `x` must be less than 2^62."] pub const unsafe fn from_u64_unchecked (x : u64) -> Self { Self (x) } # [doc = " Extract the integer value"] pub const fn into_inner (self) -> u64 { self . 0 } # [doc = " Compute the number of bytes needed to encode this value"] pub (crate) const fn size (self) -> usize { let x = self . 0 ; if x < 2u64 . pow (6) { 1 } else if x < 2u64 . pow (14) { 2 } else if x < 2u64 . pow (30) { 4 } else if x < 2u64 . pow (62) { 8 } else { panic ! ("malformed VarInt") ; } } }
};
}
