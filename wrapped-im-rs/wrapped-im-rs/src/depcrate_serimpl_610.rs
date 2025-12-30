// Generated macro for impl_610 (impl)
macro_rules! Depcrate_serimpl_610 {
() => {
// Module: crate::ser
// Provides: {"impl_610"}
// Dependencies: {}
impl < A : Serialize + Hash + Eq + Clone , S : BuildHasher + Default > Serialize for HashSet < A , S > { fn serialize < Ser > (& self , ser : Ser) -> Result < Ser :: Ok , Ser :: Error > where Ser : Serializer , { let mut s = ser . serialize_seq (Some (self . len ())) ? ; for i in self . iter () { s . serialize_element (i . deref ()) ? ; } s . end () } }
};
}
