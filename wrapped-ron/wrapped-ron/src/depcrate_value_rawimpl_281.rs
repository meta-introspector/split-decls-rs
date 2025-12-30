// Generated macro for impl_281 (impl)
macro_rules! Depcrate_value_rawimpl_281 {
() => {
// Module: crate::value::raw
// Provides: {"impl_281"}
// Dependencies: {}
impl Serialize for RawValue { fn serialize < S : ser :: Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { serializer . serialize_newtype_struct (RAW_VALUE_TOKEN , & self . ron) } }
};
}
