// Generated macro for impl_1560 (impl)
macro_rules! Depcrate_geometry_quaternion_constructionimpl_1560 {
() => {
// Module: crate::geometry::quaternion_construction
// Provides: {"impl_1560"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < T : SimdRealField + Arbitrary > Arbitrary for Quaternion < T > where Owned < T , U4 > : Send , { # [inline] fn arbitrary (g : & mut Gen) -> Self { Self :: new (T :: arbitrary (g) , T :: arbitrary (g) , T :: arbitrary (g) , T :: arbitrary (g) ,) } }
};
}
