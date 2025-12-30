// Generated macro for impl_1394 (impl)
macro_rules! Depcrate_geometry_rotationimpl_1394 {
() => {
// Module: crate::geometry::rotation
// Provides: {"impl_1394"}
// Dependencies: {}
impl < T : Scalar + hash :: Hash , const D : usize > hash :: Hash for Rotation < T , D > where < DefaultAllocator as Allocator < Const < D > , Const < D > > > :: Buffer < T > : hash :: Hash , { fn hash < H : hash :: Hasher > (& self , state : & mut H) { self . matrix . hash (state) } }
};
}
