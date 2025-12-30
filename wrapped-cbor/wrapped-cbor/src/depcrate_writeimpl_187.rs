// Generated macro for impl_187 (impl)
macro_rules! Depcrate_writeimpl_187 {
() => {
// Module: crate::write
// Provides: {"impl_187"}
// Dependencies: {}
impl < 'a > Write for SliceWrite < 'a > { type Error = error :: Error ; fn write_all (& mut self , buf : & [u8]) -> Result < () , Self :: Error > { if self . slice . len () - self . index < buf . len () { return Err (error :: Error :: scratch_too_small (self . index as u64)) ; } let end = self . index + buf . len () ; self . slice [self . index .. end] . copy_from_slice (buf) ; self . index = end ; Ok (()) } }
};
}
