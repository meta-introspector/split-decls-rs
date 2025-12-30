// Generated macro for impl_41 (impl)
macro_rules! Depcrate_derimpl_41 {
() => {
// Module: crate::der
// Provides: {"impl_41"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < C > SignatureEncoding for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { type Repr = Box < [u8] > ; fn to_vec (& self) -> Vec < u8 > { self . as_bytes () . into () } }
};
}
