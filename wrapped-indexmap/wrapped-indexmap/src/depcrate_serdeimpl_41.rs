// Generated macro for impl_41 (impl)
macro_rules! Depcrate_serdeimpl_41 {
() => {
// Module: crate::serde
// Provides: {"impl_41"}
// Dependencies: {}
impl < 'de , K , V , S > Visitor < 'de > for IndexMapVisitor < K , V , S > where K : Deserialize < 'de > + Eq + Hash , V : Deserialize < 'de > , S : Default + BuildHasher , { type Value = IndexMap < K , V , S > ; fn expecting (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { write ! (formatter , "a map") } fn visit_map < A > (self , mut map : A) -> Result < Self :: Value , A :: Error > where A : MapAccess < 'de > , { let capacity = cautious_capacity :: < K , V > (map . size_hint ()) ; let mut values = IndexMap :: with_capacity_and_hasher (capacity , S :: default ()) ; while let Some ((key , value)) = map . next_entry () ? { values . insert (key , value) ; } Ok (values) } }
};
}
