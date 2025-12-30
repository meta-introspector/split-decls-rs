// Generated macro for impl_57 (impl)
macro_rules! Depcrate_fingerprintimpl_57 {
() => {
// Module: crate::fingerprint
// Provides: {"impl_57"}
// Dependencies: {}
impl < E : Encoder > Encodable < E > for PackedFingerprint { # [inline] fn encode (& self , s : & mut E) { let copy = self . 0 ; copy . encode (s) ; } }
};
}
