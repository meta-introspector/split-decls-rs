// Generated macro for impl_109 (impl)
macro_rules! Depcrate_ed25519impl_109 {
() => {
// Module: crate::ed25519
// Provides: {"impl_109"}
// Dependencies: {}
# [cfg (feature = "random")] impl Default for Noise { # [doc = " Generates random noise."] fn default () -> Self { let mut noise = [0u8 ; Noise :: BYTES] ; getrandom :: getrandom (& mut noise) . expect ("RNG failure") ; Noise (noise) } }
};
}
