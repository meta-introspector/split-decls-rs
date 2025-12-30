// Generated macro for impl_608 (impl)
macro_rules! Depcrate_serimpl_608 {
() => {
// Module: crate::ser
// Provides: {"impl_608"}
// Dependencies: {}
impl < K , V , S > Serialize for HashMap < K , V , S > where K : Serialize + Hash + Eq + Clone , V : Serialize + Clone , S : BuildHasher + Default , { fn serialize < Ser > (& self , ser : Ser) -> Result < Ser :: Ok , Ser :: Error > where Ser : Serializer , { let mut s = ser . serialize_map (Some (self . len ())) ? ; for (k , v) in self . iter () { s . serialize_entry (k . deref () , v . deref ()) ? ; } s . end () } }
};
}
