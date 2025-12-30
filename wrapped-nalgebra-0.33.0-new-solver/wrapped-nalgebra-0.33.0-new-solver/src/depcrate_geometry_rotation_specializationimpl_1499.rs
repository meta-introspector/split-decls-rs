// Generated macro for impl_1499 (impl)
macro_rules! Depcrate_geometry_rotation_specializationimpl_1499 {
() => {
// Module: crate::geometry::rotation_specialization
// Provides: {"impl_1499"}
// Dependencies: {}
# [cfg (feature = "rand-no-std")] impl < T : SimdRealField > Distribution < Rotation3 < T > > for Standard where T :: Element : SimdRealField , OpenClosed01 : Distribution < T > , T : SampleUniform , { # [doc = " Generate a uniformly distributed random rotation."] # [inline] fn sample < 'a , R : Rng + ? Sized > (& self , rng : & mut R) -> Rotation3 < T > { let twopi = Uniform :: new (T :: zero () , T :: simd_two_pi ()) ; let theta = rng . sample (& twopi) ; let (ts , tc) = theta . simd_sin_cos () ; let a = SMatrix :: < T , 3 , 3 > :: new (tc . clone () , ts . clone () , T :: zero () , - ts , tc , T :: zero () , T :: zero () , T :: zero () , T :: one () ,) ; let phi = rng . sample (& twopi) ; let z = rng . sample (OpenClosed01) ; let (ps , pc) = phi . simd_sin_cos () ; let sqrt_z = z . clone () . simd_sqrt () ; let v = Vector3 :: new (pc * sqrt_z . clone () , ps * sqrt_z , (T :: one () - z) . simd_sqrt ()) ; let mut b = v . clone () * v . transpose () ; b += b . clone () ; b -= SMatrix :: < T , 3 , 3 > :: identity () ; Rotation3 :: from_matrix_unchecked (b * a) } }
};
}
