// Generated macro for impl_1397 (impl)
macro_rules! Depcrate_geometry_rotationimpl_1397 {
() => {
// Module: crate::geometry::rotation
// Provides: {"impl_1397"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < T : Scalar , const D : usize > Serialize for Rotation < T , D > where Owned < T , Const < D > , Const < D > > : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . matrix . serialize (serializer) } }
};
}
