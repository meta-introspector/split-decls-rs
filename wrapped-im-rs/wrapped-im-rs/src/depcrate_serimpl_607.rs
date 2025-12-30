// Generated macro for impl_607 (impl)
macro_rules! Depcrate_serimpl_607 {
() => {
// Module: crate::ser
// Provides: {"impl_607"}
// Dependencies: {}
impl < 'de , K , V , S > Deserialize < 'de > for HashMap < K , V , S > where K : Deserialize < 'de > + Hash + Eq + Clone , V : Deserialize < 'de > + Clone , S : BuildHasher + Default , { fn deserialize < D > (des : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { des . deserialize_map (MapVisitor :: < 'de , HashMap < K , V , S > , K , V > :: new ()) } }
};
}
