// Generated macro for impl_140 (impl)
macro_rules! Depcrate_serdeimpl_140 {
() => {
// Module: crate::serde
// Provides: {"impl_140"}
// Dependencies: {}
impl < 'de , K , V , S > Visitor < 'de > for ListOrderedMultimapVisitor < K , V , S > where K : Deserialize < 'de > + Eq + Hash , V : Deserialize < 'de > , S : BuildHasher + Default , { type Value = ListOrderedMultimap < K , V , S > ; fn expecting (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { write ! (formatter , "a sequence") } fn visit_seq < A > (self , mut access : A) -> Result < Self :: Value , A :: Error > where A : SeqAccess < 'de > , { let mut map = ListOrderedMultimap :: with_capacity_and_hasher (access . size_hint () . unwrap_or_default () , access . size_hint () . unwrap_or_default () , S :: default () ,) ; while let Some ((key , value)) = access . next_element () ? { let _ = map . append (key , value) ; } Ok (map) } }
};
}
