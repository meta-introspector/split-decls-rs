// Generated macro for impl_33 (impl)
macro_rules! Depcrate_derimpl_33 {
() => {
// Module: crate::der
// Provides: {"impl_33"}
// Dependencies: {}
impl < C > Debug for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "ecdsa::der::Signature<{:?}>(" , C :: default ()) ? ; for & byte in self . as_ref () { write ! (f , "{byte:02X}") ? ; } write ! (f , ")") } }
};
}
