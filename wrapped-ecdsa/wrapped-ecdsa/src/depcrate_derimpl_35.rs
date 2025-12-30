// Generated macro for impl_35 (impl)
macro_rules! Depcrate_derimpl_35 {
() => {
// Module: crate::der
// Provides: {"impl_35"}
// Dependencies: {}
impl < C > Encode for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { fn encoded_len (& self) -> der :: Result < Length > { Length :: try_from (self . len ()) } fn encode (& self , writer : & mut impl Writer) -> der :: Result < () > { writer . write (self . as_bytes ()) } }
};
}
