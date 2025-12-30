// Generated macro for impl_2239 (impl)
macro_rules! Depcrate_geometry_isometryimpl_2239 {
() => {
// Module: crate::geometry::isometry
// Provides: {"impl_2239"}
// Dependencies: {}
impl < T : Scalar + hash :: Hash , R : hash :: Hash , const D : usize > hash :: Hash for Isometry < T , R , D > where Owned < T , Const < D > > : hash :: Hash , { fn hash < H : hash :: Hasher > (& self , state : & mut H) { self . translation . hash (state) ; self . rotation . hash (state) ; } }
};
}
