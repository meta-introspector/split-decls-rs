// Generated macro for impl_5 (impl)
macro_rules! Depcrate_serdeimpl_5 {
() => {
// Module: crate::serde
// Provides: {"impl_5"}
// Dependencies: {}
impl Serialize for Tz { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { serializer . serialize_str (self . name ()) } }
};
}
