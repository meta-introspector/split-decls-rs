// Generated macro for impl_853 (impl)
macro_rules! Depcrate_srtpimpl_853 {
() => {
// Module: crate::srtp
// Provides: {"impl_853"}
// Dependencies: {}
impl SrtpProtectionProfileRef { pub fn id (& self) -> SrtpProfileId { SrtpProfileId :: from_raw (unsafe { (* self . as_ptr ()) . id }) } pub fn name (& self) -> & 'static str { unsafe { CStr :: from_ptr ((* self . as_ptr ()) . name as * const _) } . to_str () . expect ("should be UTF-8") } }
};
}
