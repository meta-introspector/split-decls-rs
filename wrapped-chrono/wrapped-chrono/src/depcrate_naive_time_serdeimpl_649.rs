// Generated macro for impl_649 (impl)
macro_rules! Depcrate_naive_time_serdeimpl_649 {
() => {
// Module: crate::naive::time::serde
// Provides: {"impl_649"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for NaiveTime { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { deserializer . deserialize_str (NaiveTimeVisitor) } }
};
}
