// Generated macro for impl_55 (impl)
macro_rules! Depcrate_lms_privateimpl_55 {
() => {
// Module: crate::lms::private
// Provides: {"impl_55"}
// Dependencies: {}
# [doc = " Tries to parse a [PrivateKey] from an exact slice"] impl < 'a , Mode : LmsMode > TryFrom < & 'a [u8] > for SigningKey < Mode > { type Error = LmsDeserializeError ; fn try_from (pk : & 'a [u8]) -> Result < Self , Self :: Error > { if pk . len () < 4 { return Err (LmsDeserializeError :: NoAlgorithm) ; } let (alg , pk) = pk . split_at (4) ; let expected = Mode :: M + ID_LEN + 8 ; if u32 :: from_be_bytes (alg . try_into () . unwrap ()) != Mode :: TYPECODE { return Err (LmsDeserializeError :: WrongAlgorithm) ; } match pk . len () . cmp (& expected) { Ordering :: Less => Err (LmsDeserializeError :: TooShort) , Ordering :: Greater => Err (LmsDeserializeError :: TooLong) , Ordering :: Equal => { let (otstype , qk) = pk . split_at (4) ; let (q , idseed) = qk . split_at (4) ; let (id , seed) = idseed . split_at (ID_LEN) ; if u32 :: from_be_bytes (otstype . try_into () . unwrap ()) != Mode :: OtsMode :: TYPECODE { return Err (LmsDeserializeError :: WrongAlgorithm) ; } let mut key = Self { q : u32 :: from_be_bytes (q . try_into () . expect ("ok")) , id : id . try_into () . expect ("ok") , seed : Array :: try_from (seed) . expect ("ok") , auth_tree : Array :: default () , } ; key . gen_pk_tree () ; Ok (key) } } } }
};
}
