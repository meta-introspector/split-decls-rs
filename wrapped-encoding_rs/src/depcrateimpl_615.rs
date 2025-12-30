// Generated macro for impl_615 (impl)
macro_rules! Depcrateimpl_615 {
() => {
// Module: crate
// Provides: {"impl_615"}
// Dependencies: {}
# [cfg (feature = "serde")] impl Serialize for Encoding { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_str (self . name) } }
};
}
