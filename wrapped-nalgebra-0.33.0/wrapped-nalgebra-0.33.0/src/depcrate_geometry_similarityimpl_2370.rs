// Generated macro for impl_2370 (impl)
macro_rules! Depcrate_geometry_similarityimpl_2370 {
() => {
// Module: crate::geometry::similarity
// Provides: {"impl_2370"}
// Dependencies: {}
impl < T : Scalar + hash :: Hash , R : hash :: Hash , const D : usize > hash :: Hash for Similarity < T , R , D > where Owned < T , Const < D > > : hash :: Hash , { fn hash < H : hash :: Hasher > (& self , state : & mut H) { self . isometry . hash (state) ; self . scaling . hash (state) ; } }
};
}
