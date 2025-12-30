// Generated macro for impl_2129 (impl)
macro_rules! Depcrate_geometry_scaleimpl_2129 {
() => {
// Module: crate::geometry::scale
// Provides: {"impl_2129"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < T : Scalar , const D : usize > Serialize for Scale < T , D > where Owned < T , Const < D > > : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . vector . serialize (serializer) } }
};
}
