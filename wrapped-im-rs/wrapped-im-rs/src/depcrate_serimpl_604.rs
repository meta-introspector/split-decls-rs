// Generated macro for impl_604 (impl)
macro_rules! Depcrate_serimpl_604 {
() => {
// Module: crate::ser
// Provides: {"impl_604"}
// Dependencies: {}
impl < A : Ord + Clone + Serialize > Serialize for OrdSet < A > { fn serialize < S > (& self , ser : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut s = ser . serialize_seq (Some (self . len ())) ? ; for i in self . iter () { s . serialize_element (i . deref ()) ? ; } s . end () } }
};
}
