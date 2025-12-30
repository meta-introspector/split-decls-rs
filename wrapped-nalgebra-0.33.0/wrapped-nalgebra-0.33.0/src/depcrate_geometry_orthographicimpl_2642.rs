// Generated macro for impl_2642 (impl)
macro_rules! Depcrate_geometry_orthographicimpl_2642 {
() => {
// Module: crate::geometry::orthographic
// Provides: {"impl_2642"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < 'a , T : RealField + Deserialize < 'a > > Deserialize < 'a > for Orthographic3 < T > { fn deserialize < Des > (deserializer : Des) -> Result < Self , Des :: Error > where Des : Deserializer < 'a > , { let matrix = Matrix4 :: < T > :: deserialize (deserializer) ? ; Ok (Self :: from_matrix_unchecked (matrix)) } }
};
}
