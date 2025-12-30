// Generated macro for impl_1493 (impl)
macro_rules! Depcrate_geometry_rotation_specializationimpl_1493 {
() => {
// Module: crate::geometry::rotation_specialization
// Provides: {"impl_1493"}
// Dependencies: {}
# [cfg (feature = "rand-no-std")] impl < T : SimdRealField > Distribution < Rotation2 < T > > for Standard where T :: Element : SimdRealField , T : SampleUniform , { # [doc = " Generate a uniformly distributed random rotation."] # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> Rotation2 < T > { let twopi = Uniform :: new (T :: zero () , T :: simd_two_pi ()) ; Rotation2 :: new (rng . sample (twopi)) } }
};
}
