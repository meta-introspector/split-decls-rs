// Generated macro for impl_605 (impl)
macro_rules! Depcrate_serimpl_605 {
() => {
// Module: crate::ser
// Provides: {"impl_605"}
// Dependencies: {}
impl < 'de , K : Deserialize < 'de > + Ord + Clone , V : Deserialize < 'de > + Clone > Deserialize < 'de > for OrdMap < K , V > { fn deserialize < D > (des : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { des . deserialize_map (MapVisitor :: < 'de , OrdMap < K , V > , K , V > :: new ()) } }
};
}
