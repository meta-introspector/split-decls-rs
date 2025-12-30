// Generated macro for impl_34 (impl)
macro_rules! Depcrate_derimpl_34 {
() => {
// Module: crate::der
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'a , C > Decode < 'a > for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { type Error = der :: Error ; fn decode < R : Reader < 'a > > (reader : & mut R) -> der :: Result < Self > { let header = Header :: peek (reader) ? ; header . tag () . assert_eq (Tag :: Sequence) ? ; let mut buf = SignatureBytes :: < C > :: default () ; let len = (header . encoded_len () ? + header . length ()) ? ; let slice = buf . get_mut (.. usize :: try_from (len) ?) . ok_or_else (| | reader . error (Tag :: Sequence . length_error ())) ? ; reader . read_into (slice) ? ; Self :: from_bytes (slice) . map_err (| _ | reader . error (Tag :: Integer . value_error ())) } }
};
}
