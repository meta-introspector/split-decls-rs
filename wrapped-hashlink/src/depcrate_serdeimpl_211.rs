// Generated macro for impl_211 (impl)
macro_rules! Depcrate_serdeimpl_211 {
() => {
// Module: crate::serde
// Provides: {"impl_211"}
// Dependencies: {}
impl < T , S > Serialize for LinkedHashSet < T , S > where T : Serialize + Eq + Hash , S : BuildHasher , { # [inline] fn serialize < U : Serializer > (& self , serializer : U) -> Result < U :: Ok , U :: Error > { let mut seq_serializer = serializer . serialize_seq (Some (self . len ())) ? ; for v in self { seq_serializer . serialize_element (v) ? ; } seq_serializer . end () } }
};
}
