// Generated macro for impl_42 (impl)
macro_rules! Depcrate_serdeimpl_42 {
() => {
// Module: crate::serde
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'de , K , V , S > Deserialize < 'de > for IndexMap < K , V , S > where K : Deserialize < 'de > + Eq + Hash , V : Deserialize < 'de > , S : Default + BuildHasher , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_map (IndexMapVisitor (PhantomData)) } }
};
}
