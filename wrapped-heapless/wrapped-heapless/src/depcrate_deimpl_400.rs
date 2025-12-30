// Generated macro for impl_400 (impl)
macro_rules! Depcrate_deimpl_400 {
() => {
// Module: crate::de
// Provides: {"impl_400"}
// Dependencies: {}
impl < 'de , T , KIND , const N : usize > Deserialize < 'de > for BinaryHeap < T , KIND , N > where T : Ord + Deserialize < 'de > , KIND : BinaryHeapKind , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct ValueVisitor < 'de , T , KIND , const N : usize > (PhantomData < (& 'de () , T , KIND) >) ; impl < 'de , T , KIND , const N : usize > de :: Visitor < 'de > for ValueVisitor < 'de , T , KIND , N > where T : Ord + Deserialize < 'de > , KIND : BinaryHeapKind , { type Value = BinaryHeap < T , KIND , N > ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : SeqAccess < 'de > , { let mut values = BinaryHeap :: new () ; while let Some (value) = seq . next_element () ? { if values . push (value) . is_err () { return Err (A :: Error :: invalid_length (values . capacity () + 1 , & self)) ? ; } } Ok (values) } } deserializer . deserialize_seq (ValueVisitor (PhantomData)) } }
};
}
