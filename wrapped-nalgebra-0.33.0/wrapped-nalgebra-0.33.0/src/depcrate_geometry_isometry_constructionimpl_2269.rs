// Generated macro for impl_2269 (impl)
macro_rules! Depcrate_geometry_isometry_constructionimpl_2269 {
() => {
// Module: crate::geometry::isometry_construction
// Provides: {"impl_2269"}
// Dependencies: {}
# [cfg (feature = "rand-no-std")] impl < T : crate :: RealField , R , const D : usize > Distribution < Isometry < T , R , D > > for Standard where R : AbstractRotation < T , D > , Standard : Distribution < T > + Distribution < R > , { # [inline] fn sample < G : Rng + ? Sized > (& self , rng : & mut G) -> Isometry < T , R , D > { Isometry :: from_parts (rng . gen () , rng . gen ()) } }
};
}
