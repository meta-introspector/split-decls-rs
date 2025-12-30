// Generated macro for impl_606 (impl)
macro_rules! Depcrate_serimpl_606 {
() => {
// Module: crate::ser
// Provides: {"impl_606"}
// Dependencies: {}
impl < K : Serialize + Ord + Clone , V : Serialize + Clone > Serialize for OrdMap < K , V > { fn serialize < S > (& self , ser : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut s = ser . serialize_map (Some (self . len ())) ? ; for (k , v) in self . iter () { s . serialize_entry (k . deref () , v . deref ()) ? ; } s . end () } }
};
}
