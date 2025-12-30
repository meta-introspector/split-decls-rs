// Generated macro for impl_126 (impl)
macro_rules! Depcrate_arrayvecimpl_126 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_126"}
// Dependencies: {}
# [cfg (feature = "borsh")] # [doc = " Requires crate feature `\"borsh\"`"] impl < T , const CAP : usize > borsh :: BorshDeserialize for ArrayVec < T , CAP > where T : borsh :: BorshDeserialize , { fn deserialize_reader < R : borsh :: io :: Read > (reader : & mut R) -> borsh :: io :: Result < Self > { let mut values = Self :: new () ; let len = < u32 as borsh :: BorshDeserialize > :: deserialize_reader (reader) ? ; for _ in 0 .. len { let elem = < T as borsh :: BorshDeserialize > :: deserialize_reader (reader) ? ; if let Err (_) = values . try_push (elem) { return Err (borsh :: io :: Error :: new (borsh :: io :: ErrorKind :: InvalidData , format ! ("Expected an array with no more than {} items" , CAP) ,)) ; } } Ok (values) } }
};
}
