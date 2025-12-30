// Generated macro for impl_2041 (impl)
macro_rules! Depcrate_geometry_translation_constructionimpl_2041 {
() => {
// Module: crate::geometry::translation_construction
// Provides: {"impl_2041"}
// Dependencies: {}
# [cfg (feature = "rand-no-std")] impl < T : Scalar , const D : usize > Distribution < Translation < T , D > > for Standard where Standard : Distribution < T > , { # [doc = " Generate an arbitrary random variate for testing purposes."] # [inline] fn sample < G : Rng + ? Sized > (& self , rng : & mut G) -> Translation < T , D > { Translation :: from (rng . gen :: < SVector < T , D > > ()) } }
};
}
