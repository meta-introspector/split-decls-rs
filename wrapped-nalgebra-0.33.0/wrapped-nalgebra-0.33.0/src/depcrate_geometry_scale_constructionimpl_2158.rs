// Generated macro for impl_2158 (impl)
macro_rules! Depcrate_geometry_scale_constructionimpl_2158 {
() => {
// Module: crate::geometry::scale_construction
// Provides: {"impl_2158"}
// Dependencies: {}
# [cfg (feature = "rand-no-std")] impl < T : Scalar , const D : usize > Distribution < Scale < T , D > > for Standard where Standard : Distribution < T > , { # [doc = " Generate an arbitrary random variate for testing purposes."] # [inline] fn sample < G : Rng + ? Sized > (& self , rng : & mut G) -> Scale < T , D > { Scale :: from (rng . gen :: < SVector < T , D > > ()) } }
};
}
