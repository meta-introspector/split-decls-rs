// Generated macro for impl_413 (impl)
macro_rules! Depcrate_serimpl_413 {
() => {
// Module: crate::ser
// Provides: {"impl_413"}
// Dependencies: {}
impl < T , S , const N : usize > Serialize for IndexSet < T , S , N > where T : Eq + Hash + Serialize , S : BuildHasher , { fn serialize < SER > (& self , serializer : SER) -> Result < SER :: Ok , SER :: Error > where SER : Serializer , { let mut seq = serializer . serialize_seq (Some (self . len ())) ? ; for element in self { seq . serialize_element (element) ? ; } seq . end () } }
};
}
