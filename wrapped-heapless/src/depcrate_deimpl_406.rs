// Generated macro for impl_406 (impl)
macro_rules! Depcrate_deimpl_406 {
() => {
// Module: crate::de
// Provides: {"impl_406"}
// Dependencies: {}
impl < 'de , K , V , const N : usize > Deserialize < 'de > for LinearMap < K , V , N > where K : Eq + Deserialize < 'de > , V : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct ValueVisitor < 'de , K , V , const N : usize > (PhantomData < (& 'de () , K , V) >) ; impl < 'de , K , V , const N : usize > de :: Visitor < 'de > for ValueVisitor < 'de , K , V , N > where K : Eq + Deserialize < 'de > , V : Deserialize < 'de > , { type Value = LinearMap < K , V , N > ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a map") } fn visit_map < A > (self , mut map : A) -> Result < Self :: Value , A :: Error > where A : MapAccess < 'de > , { let mut values = LinearMap :: new () ; while let Some ((key , value)) = map . next_entry () ? { if values . insert (key , value) . is_err () { return Err (A :: Error :: invalid_length (values . capacity () + 1 , & self)) ? ; } } Ok (values) } } deserializer . deserialize_map (ValueVisitor (PhantomData)) } }
};
}
