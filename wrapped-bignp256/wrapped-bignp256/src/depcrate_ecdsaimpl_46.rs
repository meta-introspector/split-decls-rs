// Generated macro for impl_46 (impl)
macro_rules! Depcrate_ecdsaimpl_46 {
() => {
// Module: crate::ecdsa
// Provides: {"impl_46"}
// Dependencies: {}
impl SignatureEncoding for Signature { type Repr = SignatureBytes ; fn to_bytes (& self) -> Self :: Repr { self . into () } fn encoded_len (& self) -> usize { Self :: BYTE_SIZE } }
};
}
