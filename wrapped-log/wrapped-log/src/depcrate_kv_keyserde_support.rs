// Generated macro for serde_support (module)
macro_rules! Depcrate_kv_keyserde_support {
() => {
// Module: crate::kv::key
// Provides: {"serde_support"}
// Dependencies: {}
# [cfg (feature = "kv_serde")] mod serde_support { use super :: * ; use serde :: { Serialize , Serializer } ; impl < 'a > Serialize for Key < 'a > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . key . serialize (serializer) } } }
};
}
