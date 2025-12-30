// Generated macro for impl_142 (impl)
macro_rules! Depcrate_serdeimpl_142 {
() => {
// Module: crate::serde
// Provides: {"impl_142"}
// Dependencies: {}
impl < 'de , K , V , S > Deserialize < 'de > for DashMap < K , V , S > where K : Deserialize < 'de > + Eq + Hash , V : Deserialize < 'de > , S : BuildHasher + Clone + Default , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_map (DashMapVisitor :: < K , V , S > :: new ()) } }
};
}
