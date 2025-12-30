// Generated macro for impl_138 (impl)
macro_rules! Depcrate_serdeimpl_138 {
() => {
// Module: crate::serde
// Provides: {"impl_138"}
// Dependencies: {}
impl < K , V , S > Serialize for ListOrderedMultimap < K , V , S > where K : Clone + Eq + Hash + Serialize , V : Serialize , S : BuildHasher , { fn serialize < T > (& self , serializer : T) -> Result < T :: Ok , T :: Error > where T : Serializer , { let mut seq = serializer . serialize_seq (Some (self . values_len ())) ? ; for (key , value) in self . into_iter () { seq . serialize_element (& (key , value)) ? ; } seq . end () } }
};
}
