// Generated macro for impl_528 (impl)
macro_rules! Depcrate_naive_datetime_serdeimpl_528 {
() => {
// Module: crate::naive::datetime::serde
// Provides: {"impl_528"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for NaiveDateTime { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { deserializer . deserialize_str (NaiveDateTimeVisitor) } }
};
}
