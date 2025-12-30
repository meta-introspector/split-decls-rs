// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
# [cfg (feature = "serde")] impl Serialize for Account { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { crate :: account_serialize :: serialize_account (self , serializer) } }
};
}
