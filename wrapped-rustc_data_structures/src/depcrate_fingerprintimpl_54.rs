// Generated macro for impl_54 (impl)
macro_rules! Depcrate_fingerprintimpl_54 {
() => {
// Module: crate::fingerprint
// Provides: {"impl_54"}
// Dependencies: {}
impl < D : Decoder > Decodable < D > for Fingerprint { # [inline] fn decode (d : & mut D) -> Self { Fingerprint :: from_le_bytes (d . read_raw_bytes (16) . try_into () . unwrap ()) } }
};
}
