// Generated macro for impl_191 (impl)
macro_rules! Depcrate_ots_publicimpl_191 {
() => {
// Module: crate::ots::public
// Provides: {"impl_191"}
// Dependencies: {}
# [doc = " Tries to parse a [`VerifyingKey`] from an exact slice"] impl < 'a , Mode : LmsOtsMode > TryFrom < & 'a [u8] > for VerifyingKey < Mode > { type Error = LmsDeserializeError ; fn try_from (pk : & 'a [u8]) -> Result < Self , Self :: Error > { if pk . len () < 4 { return Err (LmsDeserializeError :: NoAlgorithm) ; } let (alg , pk) = pk . split_at (4) ; let expected = Mode :: N + ID_LEN + 4 ; if u32 :: from_be_bytes (alg . try_into () . unwrap ()) != Mode :: TYPECODE { return Err (LmsDeserializeError :: WrongAlgorithm) ; } match pk . len () . cmp (& expected) { Ordering :: Less => Err (LmsDeserializeError :: TooShort) , Ordering :: Greater => Err (LmsDeserializeError :: TooLong) , Ordering :: Equal => { let (i , qk) = pk . split_at (ID_LEN) ; let (q , k) = qk . split_at (4) ; Ok (Self { q : u32 :: from_be_bytes (q . try_into () . expect ("ok")) , id : i . try_into () . expect ("ok") , k : Array :: try_from (k) . expect ("ok") , }) } } } }
};
}
