// Generated macro for impl_58 (impl)
macro_rules! Depcrate_fingerprintimpl_58 {
() => {
// Module: crate::fingerprint
// Provides: {"impl_58"}
// Dependencies: {}
impl < D : Decoder > Decodable < D > for PackedFingerprint { # [inline] fn decode (d : & mut D) -> Self { Self (Fingerprint :: decode (d)) } }
};
}
