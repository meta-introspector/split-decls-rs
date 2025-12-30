// Generated macro for impl_2668 (impl)
macro_rules! Depcrate_geometry_perspectiveimpl_2668 {
() => {
// Module: crate::geometry::perspective
// Provides: {"impl_2668"}
// Dependencies: {}
# [cfg (feature = "rand-no-std")] impl < T : RealField > Distribution < Perspective3 < T > > for Standard where Standard : Distribution < T > , { # [doc = " Generate an arbitrary random variate for testing purposes."] fn sample < R : Rng + ? Sized > (& self , r : & mut R) -> Perspective3 < T > { use crate :: base :: helper ; let znear = r . gen () ; let zfar = helper :: reject_rand (r , | x : & T | ! (x . clone () - znear . clone ()) . is_zero ()) ; let aspect = helper :: reject_rand (r , | x : & T | ! x . is_zero ()) ; Perspective3 :: new (aspect , r . gen () , znear , zfar) } }
};
}
