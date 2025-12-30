// Generated macro for impl_94 (impl)
macro_rules! Depcrate_lms_signatureimpl_94 {
() => {
// Module: crate::lms::signature
// Provides: {"impl_94"}
// Dependencies: {}
# [doc = " Tries to parse a [`Signature`] from an exact slice"] impl < Mode : LmsMode > TryFrom < & [u8] > for Signature < Mode > { type Error = LmsDeserializeError ; fn try_from (sig : & [u8]) -> Result < Self , Self :: Error > { match sig . len () . cmp (& (8 + Mode :: OtsMode :: SIG_LEN + Mode :: M * Mode :: H)) { Ordering :: Less => return Err (LmsDeserializeError :: TooShort) , Ordering :: Greater => return Err (LmsDeserializeError :: TooLong) , Ordering :: Equal => () , } ; let (q_bytes , sig) = sig . split_at (4) ; let q = u32 :: from_be_bytes (q_bytes . try_into () . unwrap ()) ; if q >= 1 << Mode :: H { return Err (LmsDeserializeError :: InvalidQ) ; } let (lmots_sig , sig) = sig . split_at (Mode :: OtsMode :: SIG_LEN) ; let ots_signature = OtsSignature :: < Mode :: OtsMode > :: try_from (lmots_sig) ? ; let (sigtype , path) = sig . split_at (4) ; if u32 :: from_be_bytes (sigtype . try_into () . unwrap ()) != Mode :: TYPECODE { return Err (LmsDeserializeError :: WrongAlgorithm) ; } let path = path . chunks_exact (Mode :: M) . map (| chunk | Array :: try_from (chunk) . expect ("size invariant violation")) . collect () ; Ok (Self { q , lmots_sig : ots_signature , path , }) } }
};
}
