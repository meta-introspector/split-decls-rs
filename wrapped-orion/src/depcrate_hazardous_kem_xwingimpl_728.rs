// Generated macro for impl_728 (impl)
macro_rules! Depcrate_hazardous_kem_xwingimpl_728 {
() => {
// Module: crate::hazardous::kem::xwing
// Provides: {"impl_728"}
// Dependencies: {}
impl DecapsulationKey { # [inline] # [doc = " Return the object as byte slice. __**Warning**__: Should not be used unless strictly"] # [doc = " needed. This __**breaks protections**__ that the type implements."] pub fn unprotected_as_bytes (& self) -> & [u8] { self . seed . unprotected_as_bytes () } }
};
}
