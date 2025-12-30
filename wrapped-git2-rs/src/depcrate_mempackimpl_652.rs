// Generated macro for impl_652 (impl)
macro_rules! Depcrate_mempackimpl_652 {
() => {
// Module: crate::mempack
// Provides: {"impl_652"}
// Dependencies: {}
impl < 'odb > Mempack < 'odb > { # [doc = " Dumps the contents of the mempack into the provided buffer."] pub fn dump (& self , repo : & Repository , buf : & mut Buf) -> Result < () , Error > { unsafe { try_call ! (raw :: git_mempack_dump (buf . raw () , repo . raw () , self . raw)) ; } Ok (()) } # [doc = " Clears all data in the mempack."] pub fn reset (& self) -> Result < () , Error > { unsafe { try_call ! (raw :: git_mempack_reset (self . raw)) ; } Ok (()) } }
};
}
