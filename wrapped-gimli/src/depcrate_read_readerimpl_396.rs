// Generated macro for impl_396 (impl)
macro_rules! Depcrate_read_readerimpl_396 {
() => {
// Module: crate::read::reader
// Provides: {"impl_396"}
// Dependencies: {}
impl ReaderOffset for u32 { # [inline] fn from_u8 (offset : u8) -> Self { u32 :: from (offset) } # [inline] fn from_u16 (offset : u16) -> Self { u32 :: from (offset) } # [inline] fn from_i16 (offset : i16) -> Self { offset as u32 } # [inline] fn from_u32 (offset : u32) -> Self { offset } # [inline] fn from_u64 (offset64 : u64) -> Result < Self > { let offset = offset64 as u32 ; if u64 :: from (offset) == offset64 { Ok (offset) } else { Err (Error :: UnsupportedOffset) } } # [inline] fn into_u64 (self) -> u64 { u64 :: from (self) } # [inline] fn wrapping_add (self , other : Self) -> Self { self . wrapping_add (other) } # [inline] fn checked_sub (self , other : Self) -> Option < Self > { self . checked_sub (other) } }
};
}
