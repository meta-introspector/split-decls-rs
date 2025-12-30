// Generated macro for impl_2126 (impl)
macro_rules! Depcrate_geometry_scaleimpl_2126 {
() => {
// Module: crate::geometry::scale
// Provides: {"impl_2126"}
// Dependencies: {}
impl < T : Scalar + hash :: Hash , const D : usize > hash :: Hash for Scale < T , D > where Owned < T , Const < D > > : hash :: Hash , { fn hash < H : hash :: Hasher > (& self , state : & mut H) { self . vector . hash (state) } }
};
}
