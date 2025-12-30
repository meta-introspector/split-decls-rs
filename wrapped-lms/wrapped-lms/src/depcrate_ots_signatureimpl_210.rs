// Generated macro for impl_210 (impl)
macro_rules! Depcrate_ots_signatureimpl_210 {
() => {
// Module: crate::ots::signature
// Provides: {"impl_210"}
// Dependencies: {}
# [doc = " Tries to parse a [`Signature`] from an exact slice"] impl < 'a , Mode : LmsOtsMode > TryFrom < & 'a [u8] > for Signature < Mode > { type Error = LmsDeserializeError ; fn try_from (sig : & 'a [u8]) -> Result < Self , Self :: Error > { if sig . len () < 4 { return Err (LmsDeserializeError :: NoAlgorithm) ; } let (alg , sig) = sig . split_at (4) ; if u32 :: from_be_bytes (alg . try_into () . unwrap ()) != Mode :: TYPECODE { return Err (LmsDeserializeError :: WrongAlgorithm) ; } let expected = Mode :: N * (Mode :: P + 1) ; match sig . len () . cmp (& expected) { Ordering :: Less => Err (LmsDeserializeError :: TooShort) , Ordering :: Greater => Err (LmsDeserializeError :: TooLong) , Ordering :: Equal => { let (c , y) = sig . split_at (Mode :: N) ; let c = Array :: try_from (c) . expect ("size invariant violation") ; let ys = y . chunks_exact (Mode :: N) . map (| chunk | Array :: try_from (chunk) . expect ("size invariant violation")) ; debug_assert ! (ys . len () == Mode :: P) ; let y = Array :: from_iter (ys) ; Ok (Self { c , y }) } } } }
};
}
