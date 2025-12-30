// Generated macro for impl_260 (impl)
macro_rules! Depcrate_common_ageimpl_260 {
() => {
// Module: crate::common::age
// Provides: {"impl_260"}
// Dependencies: {}
impl Age { # [doc = " Creates a new `Age` header from the specified number of whole seconds."] pub fn from_secs (secs : u64) -> Self { Self (Seconds :: from_secs (secs)) } # [doc = " Returns the number of seconds for this `Age` header."] pub fn as_secs (& self) -> u64 { self . 0 . as_u64 () } }
};
}
