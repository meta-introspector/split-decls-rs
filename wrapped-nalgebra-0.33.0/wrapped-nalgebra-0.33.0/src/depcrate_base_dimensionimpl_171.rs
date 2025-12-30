// Generated macro for impl_171 (impl)
macro_rules! Depcrate_base_dimensionimpl_171 {
() => {
// Module: crate::base::dimension
// Provides: {"impl_171"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < 'de > Deserialize < 'de > for Dyn { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { usize :: deserialize (deserializer) . map (| x | Dyn (x)) } }
};
}
