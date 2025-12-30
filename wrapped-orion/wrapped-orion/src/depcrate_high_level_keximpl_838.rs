// Generated macro for impl_838 (impl)
macro_rules! Depcrate_high_level_keximpl_838 {
() => {
// Module: crate::high_level::kex
// Provides: {"impl_838"}
// Dependencies: {}
impl SessionKeys { # [doc = " Get the shared secret intended to be used for receiving data from the other party."] pub fn receiving (& self) -> & SecretKey { & self . rx } # [doc = " Get the shared secret intended to be used for transporting data to the other party."] pub fn transport (& self) -> & SecretKey { & self . tx } }
};
}
