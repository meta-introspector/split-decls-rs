// Generated macro for impl_97 (impl)
macro_rules! Depcrateimpl_97 {
() => {
// Module: crate
// Provides: {"impl_97"}
// Dependencies: {}
impl SignatureEncoding for Signature { type Repr = Box < [u8] > ; fn to_bytes (& self) -> Box < [u8] > { SignatureEncoding :: to_vec (self) . into_boxed_slice () } fn to_vec (& self) -> Vec < u8 > { self . to_der () . expect ("DER encoding error") } }
};
}
