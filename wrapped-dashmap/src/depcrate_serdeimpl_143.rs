// Generated macro for impl_143 (impl)
macro_rules! Depcrate_serdeimpl_143 {
() => {
// Module: crate::serde
// Provides: {"impl_143"}
// Dependencies: {}
impl < K , V , H > Serialize for DashMap < K , V , H > where K : Serialize + Eq + Hash , V : Serialize , H : BuildHasher + Clone , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut map = serializer . serialize_map (Some (self . len ())) ? ; for ref_multi in self . iter () { map . serialize_entry (ref_multi . key () , ref_multi . value ()) ? ; } map . end () } }
};
}
