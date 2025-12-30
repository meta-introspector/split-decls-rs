// Generated macro for impl_185 (impl)
macro_rules! Depcrate_base_dimensionimpl_185 {
() => {
// Module: crate::base::dimension
// Provides: {"impl_185"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < 'de , const D : usize > Deserialize < 'de > for Const < D > { fn deserialize < Des > (deserializer : Des) -> Result < Self , Des :: Error > where Des : Deserializer < 'de > , { < () > :: deserialize (deserializer) . map (| _ | Const :: < D >) } }
};
}
