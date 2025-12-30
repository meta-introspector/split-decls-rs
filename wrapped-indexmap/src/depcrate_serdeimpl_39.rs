// Generated macro for impl_39 (impl)
macro_rules! Depcrate_serdeimpl_39 {
() => {
// Module: crate::serde
// Provides: {"impl_39"}
// Dependencies: {}
impl < K , V , S > Serialize for IndexMap < K , V , S > where K : Serialize , V : Serialize , { fn serialize < T > (& self , serializer : T) -> Result < T :: Ok , T :: Error > where T : Serializer , { serializer . collect_map (self) } }
};
}
