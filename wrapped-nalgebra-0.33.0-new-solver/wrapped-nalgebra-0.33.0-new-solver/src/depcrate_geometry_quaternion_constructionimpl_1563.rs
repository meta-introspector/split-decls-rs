// Generated macro for impl_1563 (impl)
macro_rules! Depcrate_geometry_quaternion_constructionimpl_1563 {
() => {
// Module: crate::geometry::quaternion_construction
// Provides: {"impl_1563"}
// Dependencies: {}
# [cfg (feature = "rand-no-std")] impl < T : SimdRealField > Distribution < UnitQuaternion < T > > for Standard where T :: Element : SimdRealField , OpenClosed01 : Distribution < T > , T : SampleUniform , { # [doc = " Generate a uniformly distributed random rotation quaternion."] # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> UnitQuaternion < T > { let x0 = rng . sample (OpenClosed01) ; let twopi = Uniform :: new (T :: zero () , T :: simd_two_pi ()) ; let theta1 = rng . sample (& twopi) ; let theta2 = rng . sample (& twopi) ; let s1 = theta1 . clone () . simd_sin () ; let c1 = theta1 . simd_cos () ; let s2 = theta2 . clone () . simd_sin () ; let c2 = theta2 . simd_cos () ; let r1 = (T :: one () - x0 . clone ()) . simd_sqrt () ; let r2 = x0 . simd_sqrt () ; Unit :: new_unchecked (Quaternion :: new (s1 * r1 . clone () , c1 * r1 , s2 * r2 . clone () , c2 * r2 ,)) } }
};
}
