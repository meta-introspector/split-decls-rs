// Generated macro for impl_2669 (impl)
macro_rules! Depcrate_geometry_perspectiveimpl_2669 {
() => {
// Module: crate::geometry::perspective
// Provides: {"impl_2669"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < T : RealField + Arbitrary > Arbitrary for Perspective3 < T > { fn arbitrary (g : & mut Gen) -> Self { use crate :: base :: helper ; let znear : T = Arbitrary :: arbitrary (g) ; let zfar = helper :: reject (g , | x : & T | ! (x . clone () - znear . clone ()) . is_zero ()) ; let aspect = helper :: reject (g , | x : & T | ! x . is_zero ()) ; Self :: new (aspect , Arbitrary :: arbitrary (g) , znear , zfar) } }
};
}
