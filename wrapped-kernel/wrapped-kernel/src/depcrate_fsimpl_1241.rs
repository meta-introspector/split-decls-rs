// Generated macro for impl_1241 (impl)
macro_rules! Depcrate_fsimpl_1241 {
() => {
// Module: crate::fs
// Provides: {"impl_1241"}
// Dependencies: {}
impl Read for File { fn read (& mut self , buf : & mut [u8]) -> Result < usize , Self :: Error > { let buf = unsafe { core :: slice :: from_raw_parts_mut (buf . as_mut_ptr () . cast () , buf . len ()) } ; fd :: read (self . fd , buf) } }
};
}
