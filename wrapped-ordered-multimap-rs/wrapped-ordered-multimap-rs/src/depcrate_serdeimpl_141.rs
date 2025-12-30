// Generated macro for impl_141 (impl)
macro_rules! Depcrate_serdeimpl_141 {
() => {
// Module: crate::serde
// Provides: {"impl_141"}
// Dependencies: {}
impl < 'de , K , V , S > Deserialize < 'de > for ListOrderedMultimap < K , V , S > where K : Deserialize < 'de > + Eq + Hash , V : Deserialize < 'de > , S : BuildHasher + Default , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_seq (ListOrderedMultimapVisitor (PhantomData)) } }
};
}
