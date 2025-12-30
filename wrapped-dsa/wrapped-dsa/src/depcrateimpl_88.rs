// Generated macro for impl_88 (impl)
macro_rules! Depcrateimpl_88 {
() => {
// Module: crate
// Provides: {"impl_88"}
// Dependencies: {}
impl SignatureEncoding for Signature { type Repr = Box < [u8] > ; fn to_bytes (& self) -> Box < [u8] > { SignatureEncoding :: to_vec (self) . into_boxed_slice () } fn to_vec (& self) -> Vec < u8 > { self . to_der () . expect ("DER encoding error") } }
};
}
