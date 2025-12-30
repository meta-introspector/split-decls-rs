// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
# [cfg (feature = "serde")] impl Serialize for AccountSharedData { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { crate :: account_serialize :: serialize_account (self , serializer) } }
};
}
