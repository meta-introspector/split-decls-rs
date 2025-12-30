// Generated macro for impl_1559 (impl)
macro_rules! Depcrate_geometry_quaternion_constructionimpl_1559 {
() => {
// Module: crate::geometry::quaternion_construction
// Provides: {"impl_1559"}
// Dependencies: {}
# [cfg (feature = "rand-no-std")] impl < T : SimdRealField > Distribution < Quaternion < T > > for Standard where Standard : Distribution < T > , { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> Quaternion < T > { Quaternion :: new (rng . gen () , rng . gen () , rng . gen () , rng . gen ()) } }
};
}
