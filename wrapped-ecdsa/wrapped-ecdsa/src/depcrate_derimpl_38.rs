// Generated macro for impl_38 (impl)
macro_rules! Depcrate_derimpl_38 {
() => {
// Module: crate::der
// Provides: {"impl_38"}
// Dependencies: {}
impl < C > TryFrom < & [u8] > for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { type Error = Error ; fn try_from (input : & [u8]) -> Result < Self > { Self :: from_bytes (input) } }
};
}
