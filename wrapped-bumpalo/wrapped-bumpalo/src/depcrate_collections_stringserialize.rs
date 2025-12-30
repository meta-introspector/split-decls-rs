// Generated macro for serialize (module)
macro_rules! Depcrate_collections_stringserialize {
() => {
// Module: crate::collections::string
// Provides: {"serialize"}
// Dependencies: {}
# [cfg (feature = "serde")] mod serialize { use super :: * ; use serde :: { Serialize , Serializer } ; impl < 'bump > Serialize for String < 'bump > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_str (& self) } } }
};
}
