// Generated macro for impl_402 (impl)
macro_rules! Depcrate_deimpl_402 {
() => {
// Module: crate::de
// Provides: {"impl_402"}
// Dependencies: {}
impl < 'de , T , LenT : LenType , const N : usize > Deserialize < 'de > for Vec < T , N , LenT > where T : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct ValueVisitor < 'de , T , LenT : LenType , const N : usize > (PhantomData < (& 'de () , T , LenT) >) ; impl < 'de , T , LenT , const N : usize > serde_core :: de :: Visitor < 'de > for ValueVisitor < 'de , T , LenT , N > where T : Deserialize < 'de > , LenT : LenType , { type Value = Vec < T , N , LenT > ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : SeqAccess < 'de > , { let mut values = Vec :: new () ; while let Some (value) = seq . next_element () ? { if values . push (value) . is_err () { return Err (A :: Error :: invalid_length (values . capacity () + 1 , & self)) ? ; } } Ok (values) } } deserializer . deserialize_seq (ValueVisitor (PhantomData)) } }
};
}
