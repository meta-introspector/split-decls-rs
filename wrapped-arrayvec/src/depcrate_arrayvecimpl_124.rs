// Generated macro for impl_124 (impl)
macro_rules! Depcrate_arrayvecimpl_124 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_124"}
// Dependencies: {}
# [cfg (feature = "serde")] # [doc = " Requires crate feature `\"serde\"`"] impl < 'de , T : Deserialize < 'de > , const CAP : usize > Deserialize < 'de > for ArrayVec < T , CAP > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { use serde :: de :: { Error , SeqAccess , Visitor } ; use std :: marker :: PhantomData ; struct ArrayVecVisitor < 'de , T : Deserialize < 'de > , const CAP : usize > (PhantomData < (& 'de () , [T ; CAP]) > ,) ; impl < 'de , T : Deserialize < 'de > , const CAP : usize > Visitor < 'de > for ArrayVecVisitor < 'de , T , CAP > { type Value = ArrayVec < T , CAP > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "an array with no more than {} items" , CAP) } fn visit_seq < SA > (self , mut seq : SA) -> Result < Self :: Value , SA :: Error > where SA : SeqAccess < 'de > , { let mut values = ArrayVec :: < T , CAP > :: new () ; while let Some (value) = seq . next_element () ? { if let Err (_) = values . try_push (value) { return Err (SA :: Error :: invalid_length (CAP + 1 , & self)) ; } } Ok (values) } } deserializer . deserialize_seq (ArrayVecVisitor :: < T , CAP > (PhantomData)) } }
};
}
