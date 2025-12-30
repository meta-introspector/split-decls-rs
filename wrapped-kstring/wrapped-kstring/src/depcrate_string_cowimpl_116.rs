// Generated macro for impl_116 (impl)
macro_rules! Depcrate_string_cowimpl_116 {
() => {
// Module: crate::string_cow
// Provides: {"impl_116"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < B : crate :: backend :: HeapStr > serde :: Serialize for KStringCowBase < '_ , B > { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serializer . serialize_str (self . as_str ()) } }
};
}
