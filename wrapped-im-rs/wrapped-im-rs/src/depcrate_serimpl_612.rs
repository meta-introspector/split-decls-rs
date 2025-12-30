// Generated macro for impl_612 (impl)
macro_rules! Depcrate_serimpl_612 {
() => {
// Module: crate::ser
// Provides: {"impl_612"}
// Dependencies: {}
impl < A : Clone + Serialize > Serialize for Vector < A > { fn serialize < S > (& self , ser : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut s = ser . serialize_seq (Some (self . len ())) ? ; for i in self . iter () { s . serialize_element (i . deref ()) ? ; } s . end () } }
};
}
