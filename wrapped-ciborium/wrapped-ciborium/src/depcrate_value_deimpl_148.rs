// Generated macro for impl_148 (impl)
macro_rules! Depcrate_value_deimpl_148 {
() => {
// Module: crate::value::de
// Provides: {"impl_148"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for Value { # [inline] fn deserialize < D : de :: Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { deserializer . deserialize_any (Visitor) } }
};
}
