// Generated macro for impl_2645 (impl)
macro_rules! Depcrate_geometry_orthographicimpl_2645 {
() => {
// Module: crate::geometry::orthographic
// Provides: {"impl_2645"}
// Dependencies: {}
# [cfg (feature = "rand-no-std")] impl < T : RealField > Distribution < Orthographic3 < T > > for Standard where Standard : Distribution < T > , { # [doc = " Generate an arbitrary random variate for testing purposes."] fn sample < R : Rng + ? Sized > (& self , r : & mut R) -> Orthographic3 < T > { use crate :: base :: helper ; let left = r . gen () ; let right = helper :: reject_rand (r , | x : & T | * x > left) ; let bottom = r . gen () ; let top = helper :: reject_rand (r , | x : & T | * x > bottom) ; let znear = r . gen () ; let zfar = helper :: reject_rand (r , | x : & T | * x > znear) ; Orthographic3 :: new (left , right , bottom , top , znear , zfar) } }
};
}
