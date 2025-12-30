// Generated macro for impl_147 (impl)
macro_rules! Depcrate_serdeimpl_147 {
() => {
// Module: crate::serde
// Provides: {"impl_147"}
// Dependencies: {}
impl < 'de , K , S > Deserialize < 'de > for DashSet < K , S > where K : Deserialize < 'de > + Eq + Hash , S : BuildHasher + Clone + Default , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_seq (DashSetVisitor :: < K , S > :: new ()) } }
};
}
