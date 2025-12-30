// Generated macro for impl_209 (impl)
macro_rules! Depcrate_serdeimpl_209 {
() => {
// Module: crate::serde
// Provides: {"impl_209"}
// Dependencies: {}
impl < K , V , S > Serialize for LinkedHashMap < K , V , S > where K : Serialize + Eq + Hash , V : Serialize , S : BuildHasher , { # [inline] fn serialize < T : Serializer > (& self , serializer : T) -> Result < T :: Ok , T :: Error > { let mut map_serializer = serializer . serialize_map (Some (self . len ())) ? ; for (k , v) in self { map_serializer . serialize_key (k) ? ; map_serializer . serialize_value (v) ? ; } map_serializer . end () } }
};
}
