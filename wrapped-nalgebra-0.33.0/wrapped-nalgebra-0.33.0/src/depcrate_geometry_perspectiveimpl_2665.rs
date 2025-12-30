// Generated macro for impl_2665 (impl)
macro_rules! Depcrate_geometry_perspectiveimpl_2665 {
() => {
// Module: crate::geometry::perspective
// Provides: {"impl_2665"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < 'a , T : RealField + Deserialize < 'a > > Deserialize < 'a > for Perspective3 < T > { fn deserialize < Des > (deserializer : Des) -> Result < Self , Des :: Error > where Des : Deserializer < 'a > , { let matrix = Matrix4 :: < T > :: deserialize (deserializer) ? ; Ok (Self :: from_matrix_unchecked (matrix)) } }
};
}
