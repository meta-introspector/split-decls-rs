// Generated macro for impl_2646 (impl)
macro_rules! Depcrate_geometry_orthographicimpl_2646 {
() => {
// Module: crate::geometry::orthographic
// Provides: {"impl_2646"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < T : RealField + Arbitrary > Arbitrary for Orthographic3 < T > where Matrix4 < T > : Send , { fn arbitrary (g : & mut Gen) -> Self { use crate :: base :: helper ; let left = Arbitrary :: arbitrary (g) ; let right = helper :: reject (g , | x : & T | * x > left) ; let bottom = Arbitrary :: arbitrary (g) ; let top = helper :: reject (g , | x : & T | * x > bottom) ; let znear = Arbitrary :: arbitrary (g) ; let zfar = helper :: reject (g , | x : & T | * x > znear) ; Self :: new (left , right , bottom , top , znear , zfar) } }
};
}
