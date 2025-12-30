// Generated macro for impl_68 (impl)
macro_rules! Depcrate_stringimpl_68 {
() => {
// Module: crate::string
// Provides: {"impl_68"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < B : crate :: backend :: HeapStr > serde :: Serialize for KStringBase < B > { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serializer . serialize_str (self . as_str ()) } }
};
}
