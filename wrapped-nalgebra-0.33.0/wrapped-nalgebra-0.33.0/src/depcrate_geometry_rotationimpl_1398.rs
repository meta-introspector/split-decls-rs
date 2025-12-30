// Generated macro for impl_1398 (impl)
macro_rules! Depcrate_geometry_rotationimpl_1398 {
() => {
// Module: crate::geometry::rotation
// Provides: {"impl_1398"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < 'a , T : Scalar , const D : usize > Deserialize < 'a > for Rotation < T , D > where Owned < T , Const < D > , Const < D > > : Deserialize < 'a > , { fn deserialize < Des > (deserializer : Des) -> Result < Self , Des :: Error > where Des : Deserializer < 'a > , { let matrix = SMatrix :: < T , D , D > :: deserialize (deserializer) ? ; Ok (Self :: from_matrix_unchecked (matrix)) } }
};
}
