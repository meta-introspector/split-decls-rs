// Generated macro for impl_49 (impl)
macro_rules! Depcrate_fingerprintimpl_49 {
() => {
// Module: crate::fingerprint
// Provides: {"impl_49"}
// Dependencies: {}
impl < H : Hasher > FingerprintHasher for H { # [inline] default fn write_fingerprint (& mut self , fingerprint : & Fingerprint) { self . write_u64 (fingerprint . 0) ; self . write_u64 (fingerprint . 1) ; } }
};
}
