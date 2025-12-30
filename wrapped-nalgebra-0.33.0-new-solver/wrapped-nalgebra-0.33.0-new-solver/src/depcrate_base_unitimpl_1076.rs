// Generated macro for impl_1076 (impl)
macro_rules! Depcrate_base_unitimpl_1076 {
() => {
// Module: crate::base::unit
// Provides: {"impl_1076"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < 'de , T : Deserialize < 'de > > Deserialize < 'de > for Unit < T > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { T :: deserialize (deserializer) . map (| x | Unit { value : x }) } }
};
}
