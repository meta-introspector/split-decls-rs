// Generated macro for VersionedPairing (enum)
macro_rules! Depcrate_pairingVersionedPairing {
() => {
// Module: crate::pairing
// Provides: {"VersionedPairing"}
// Dependencies: {}
# [doc = " The version enum used to version changes to the `alt_bn128_pairing` syscall."] # [cfg (not (target_os = "solana"))] pub enum VersionedPairing { V0 , # [doc = " SIMD-0334 - Fix alt_bn128_pairing Syscall Length Check"] V1 , }
};
}
