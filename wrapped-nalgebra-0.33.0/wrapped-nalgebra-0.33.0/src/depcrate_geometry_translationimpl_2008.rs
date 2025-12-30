// Generated macro for impl_2008 (impl)
macro_rules! Depcrate_geometry_translationimpl_2008 {
() => {
// Module: crate::geometry::translation
// Provides: {"impl_2008"}
// Dependencies: {}
impl < T : Scalar + hash :: Hash , const D : usize > hash :: Hash for Translation < T , D > where Owned < T , Const < D > > : hash :: Hash , { fn hash < H : hash :: Hasher > (& self , state : & mut H) { self . vector . hash (state) } }
};
}
