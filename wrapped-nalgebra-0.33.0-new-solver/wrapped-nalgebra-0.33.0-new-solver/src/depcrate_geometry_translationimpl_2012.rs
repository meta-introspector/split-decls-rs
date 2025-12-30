// Generated macro for impl_2012 (impl)
macro_rules! Depcrate_geometry_translationimpl_2012 {
() => {
// Module: crate::geometry::translation
// Provides: {"impl_2012"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < 'a , T : Scalar , const D : usize > Deserialize < 'a > for Translation < T , D > where Owned < T , Const < D > > : Deserialize < 'a > , { fn deserialize < Des > (deserializer : Des) -> Result < Self , Des :: Error > where Des : Deserializer < 'a > , { let matrix = SVector :: < T , D > :: deserialize (deserializer) ? ; Ok (Translation :: from (matrix)) } }
};
}
