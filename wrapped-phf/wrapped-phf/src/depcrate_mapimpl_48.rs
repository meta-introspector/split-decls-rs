// Generated macro for impl_48 (impl)
macro_rules! Depcrate_mapimpl_48 {
() => {
// Module: crate::map
// Provides: {"impl_48"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < K , V > Serialize for Map < K , V > where K : Serialize , V : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut map = serializer . serialize_map (Some (self . len ())) ? ; for (k , v) in self . entries () { map . serialize_entry (k , v) ? ; } map . end () } }
};
}
