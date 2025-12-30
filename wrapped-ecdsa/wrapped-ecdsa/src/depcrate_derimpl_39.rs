// Generated macro for impl_39 (impl)
macro_rules! Depcrate_derimpl_39 {
() => {
// Module: crate::der
// Provides: {"impl_39"}
// Dependencies: {}
impl < C > TryFrom < Signature < C > > for crate :: Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { type Error = Error ; fn try_from (sig : Signature < C >) -> Result < super :: Signature < C > > { let mut bytes = super :: SignatureBytes :: < C > :: default () ; let r_begin = C :: FieldBytesSize :: USIZE . saturating_sub (sig . r () . len ()) ; let s_begin = bytes . len () . saturating_sub (sig . s () . len ()) ; bytes [r_begin .. C :: FieldBytesSize :: USIZE] . copy_from_slice (sig . r ()) ; bytes [s_begin ..] . copy_from_slice (sig . s ()) ; Self :: try_from (bytes . as_slice ()) } }
};
}
