// Generated macro for impl_399 (impl)
macro_rules! Depcrate_read_readerimpl_399 {
() => {
// Module: crate::read::reader
// Provides: {"impl_399"}
// Dependencies: {}
impl ReaderAddress for u64 { # [inline] fn add_sized (self , length : u64 , size : u8) -> Result < Self > { let address = self . checked_add (length) . ok_or (Error :: AddressOverflow) ? ; let mask = Self :: ones_sized (size) ; if address & ! mask != 0 { return Err (Error :: AddressOverflow) ; } Ok (address) } # [inline] fn wrapping_add_sized (self , length : u64 , size : u8) -> Self { let mask = Self :: ones_sized (size) ; self . wrapping_add (length) & mask } # [inline] fn zeros () -> Self { 0 } # [inline] fn ones_sized (size : u8) -> Self { ! 0 >> (64 - size * 8) } }
};
}
