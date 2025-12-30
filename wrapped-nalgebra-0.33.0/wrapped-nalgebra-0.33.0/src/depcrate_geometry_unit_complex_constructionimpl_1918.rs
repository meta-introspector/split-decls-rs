// Generated macro for impl_1918 (impl)
macro_rules! Depcrate_geometry_unit_complex_constructionimpl_1918 {
() => {
// Module: crate::geometry::unit_complex_construction
// Provides: {"impl_1918"}
// Dependencies: {}
# [doc = " # Identity"] impl < T : SimdRealField > UnitComplex < T > where T :: Element : SimdRealField , { # [doc = " The unit complex number multiplicative identity."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::UnitComplex;"] # [doc = " let rot1 = UnitComplex::identity();"] # [doc = " let rot2 = UnitComplex::new(1.7);"] # [doc = ""] # [doc = " assert_eq!(rot1 * rot2, rot2);"] # [doc = " assert_eq!(rot2 * rot1, rot2);"] # [doc = " ```"] # [inline] pub fn identity () -> Self { Self :: new_unchecked (Complex :: new (T :: one () , T :: zero ())) } }
};
}
