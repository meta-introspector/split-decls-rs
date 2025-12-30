// Generated macro for impl_151 (impl)
macro_rules! Depcrate_string_refimpl_151 {
() => {
// Module: crate::string_ref
// Provides: {"impl_151"}
// Dependencies: {}
# [cfg (feature = "serde")] impl serde :: Serialize for KStringRef < '_ > { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serializer . serialize_str (self . as_str ()) } }
};
}
