// Generated macro for impl_2130 (impl)
macro_rules! Depcrate_geometry_scaleimpl_2130 {
() => {
// Module: crate::geometry::scale
// Provides: {"impl_2130"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < 'a , T : Scalar , const D : usize > Deserialize < 'a > for Scale < T , D > where Owned < T , Const < D > > : Deserialize < 'a > , { fn deserialize < Des > (deserializer : Des) -> Result < Self , Des :: Error > where Des : Deserializer < 'a > , { let matrix = SVector :: < T , D > :: deserialize (deserializer) ? ; Ok (Scale :: from (matrix)) } }
};
}
