// Generated macro for impl_1302 (impl)
macro_rules! Depcrate_geometry_point_constructionimpl_1302 {
() => {
// Module: crate::geometry::point_construction
// Provides: {"impl_1302"}
// Dependencies: {}
# [cfg (feature = "rand-no-std")] impl < T : Scalar , D : DimName > Distribution < OPoint < T , D > > for Standard where Standard : Distribution < T > , DefaultAllocator : Allocator < D > , { # [doc = " Generate a `Point` where each coordinate is an independent variate from `[0, 1)`."] # [inline] fn sample < 'a , G : Rng + ? Sized > (& self , rng : & mut G) -> OPoint < T , D > { OPoint :: from (rng . gen :: < OVector < T , D > > ()) } }
};
}
