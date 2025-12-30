// Generated macro for impl_189 (impl)
macro_rules! Depcrateimpl_189 {
() => {
// Module: crate
// Provides: {"impl_189"}
// Dependencies: {}
# [cfg (feature = "serde")] # [cfg_attr (docsrs , doc (cfg (feature = "serde")))] impl < T , const N : usize > Serialize for SmallVec < T , N > where T : Serialize , { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { let mut state = serializer . serialize_seq (Some (self . len ())) ? ; for item in self { state . serialize_element (item) ? ; } state . end () } }
};
}
