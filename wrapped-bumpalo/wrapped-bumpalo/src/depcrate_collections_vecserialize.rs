// Generated macro for serialize (module)
macro_rules! Depcrate_collections_vecserialize {
() => {
// Module: crate::collections::vec
// Provides: {"serialize"}
// Dependencies: {}
# [cfg (feature = "serde")] mod serialize { use super :: * ; use serde :: { ser :: SerializeSeq , Serialize , Serializer } ; impl < 'a , T > Serialize for Vec < 'a , T > where T : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut seq = serializer . serialize_seq (Some (self . len)) ? ; for e in self . iter () { seq . serialize_element (e) ? ; } seq . end () } } }
};
}
