// Generated macro for impl_78 (impl)
macro_rules! Depcrate_lms_publicimpl_78 {
() => {
// Module: crate::lms::public
// Provides: {"impl_78"}
// Dependencies: {}
# [doc = " Tries to parse a [`VerifyingKey`] from an exact slice"] impl < 'a , Mode : LmsMode > TryFrom < & 'a [u8] > for VerifyingKey < Mode > { type Error = LmsDeserializeError ; fn try_from (pk : & 'a [u8]) -> Result < Self , Self :: Error > { let expected_len = Mode :: M + ID_LEN + 8 ; match pk . len () . cmp (& expected_len) { Ordering :: Less => return Err (LmsDeserializeError :: TooShort) , Ordering :: Greater => return Err (LmsDeserializeError :: TooLong) , Ordering :: Equal => () , } ; let (alg , pk) = pk . split_at (4) ; if u32 :: from_be_bytes (alg . try_into () . unwrap ()) != Mode :: TYPECODE { return Err (LmsDeserializeError :: WrongAlgorithm) ; } let (otstype , id_k) = pk . split_at (4) ; if u32 :: from_be_bytes (otstype . try_into () . unwrap ()) != Mode :: OtsMode :: TYPECODE { return Err (LmsDeserializeError :: WrongAlgorithm) ; } let (id , k) = id_k . split_at (ID_LEN) ; Ok (Self { id : id . try_into () . unwrap () , k : Array :: try_from (k) . expect ("size invariant violation") , }) } }
};
}
