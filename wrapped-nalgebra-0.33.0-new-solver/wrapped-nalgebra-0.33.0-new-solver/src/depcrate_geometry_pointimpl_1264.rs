// Generated macro for impl_1264 (impl)
macro_rules! Depcrate_geometry_pointimpl_1264 {
() => {
// Module: crate::geometry::point
// Provides: {"impl_1264"}
// Dependencies: {}
impl < T : Scalar + hash :: Hash , D : DimName > hash :: Hash for OPoint < T , D > where DefaultAllocator : Allocator < D > , { fn hash < H : hash :: Hasher > (& self , state : & mut H) { self . coords . hash (state) } }
};
}
