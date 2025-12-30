// Generated macro for impl_16 (impl)
macro_rules! Depcrate_spanimpl_16 {
() => {
// Module: crate::span
// Provides: {"impl_16"}
// Dependencies: {}
impl Random for u128 { fn random () -> Self { let mut output = [0 ; 16] ; hash_stuff () . squeeze (& mut output) ; Self :: from_ne_bytes (output) } }
};
}
