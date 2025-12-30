// Generated macro for impl_15 (impl)
macro_rules! Depcrate_spanimpl_15 {
() => {
// Module: crate::span
// Provides: {"impl_15"}
// Dependencies: {}
impl Random for u64 { fn random () -> Self { let mut output = [0 ; 8] ; hash_stuff () . squeeze (& mut output) ; Self :: from_ne_bytes (output) } }
};
}
