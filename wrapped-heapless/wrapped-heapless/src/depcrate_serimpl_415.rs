// Generated macro for impl_415 (impl)
macro_rules! Depcrate_serimpl_415 {
() => {
// Module: crate::ser
// Provides: {"impl_415"}
// Dependencies: {}
impl < T , S : VecStorage < T > + ? Sized > Serialize for DequeInner < T , S > where T : Serialize , { fn serialize < SER > (& self , serializer : SER) -> Result < SER :: Ok , SER :: Error > where SER : Serializer , { let mut seq = serializer . serialize_seq (Some (self . storage_len ())) ? ; for element in self { seq . serialize_element (element) ? ; } seq . end () } }
};
}
