// Generated macro for impl_31 (impl)
macro_rules! Depcrate_derimpl_31 {
() => {
// Module: crate::der
// Provides: {"impl_31"}
// Dependencies: {}
impl < C > AsRef < [u8] > for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { fn as_ref (& self) -> & [u8] { self . as_bytes () } }
};
}
