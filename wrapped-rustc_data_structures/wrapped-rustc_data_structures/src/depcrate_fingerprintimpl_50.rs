// Generated macro for impl_50 (impl)
macro_rules! Depcrate_fingerprintimpl_50 {
() => {
// Module: crate::fingerprint
// Provides: {"impl_50"}
// Dependencies: {}
impl FingerprintHasher for crate :: unhash :: Unhasher { # [inline] fn write_fingerprint (& mut self , fingerprint : & Fingerprint) { self . write_u64 (fingerprint . 0 . wrapping_add (fingerprint . 1)) ; } }
};
}
