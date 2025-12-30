// Generated macro for serialize (module)
macro_rules! Depcrate_boxedserialize {
() => {
// Module: crate::boxed
// Provides: {"serialize"}
// Dependencies: {}
# [cfg (feature = "serde")] mod serialize { use super :: * ; use serde :: { Serialize , Serializer } ; impl < 'a , T > Serialize for Box < 'a , T > where T : Serialize , { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { T :: serialize (self , serializer) } } }
};
}
