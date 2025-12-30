// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
impl DerefMut for MmapMut { # [inline] fn deref_mut (& mut self) -> & mut [u8] { unsafe { slice :: from_raw_parts_mut (self . inner . mut_ptr () , self . inner . len ()) } } }
};
}
