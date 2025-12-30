// Generated macro for impl_2011 (impl)
macro_rules! Depcrate_geometry_translationimpl_2011 {
() => {
// Module: crate::geometry::translation
// Provides: {"impl_2011"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < T : Scalar , const D : usize > Serialize for Translation < T , D > where Owned < T , Const < D > > : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . vector . serialize (serializer) } }
};
}
