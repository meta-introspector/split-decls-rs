// Generated macro for impl_2276 (impl)
macro_rules! Depcrate_geometry_isometry_constructionimpl_2276 {
() => {
// Module: crate::geometry::isometry_construction
// Provides: {"impl_2276"}
// Dependencies: {}
impl < T : SimdRealField > IsometryMatrix3 < T > where T :: Element : SimdRealField , { basic_isometry_construction_impl ! (Rotation3 < T >) ; # [doc = " Cast the components of `self` to another type."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::IsometryMatrix3;"] # [doc = " let iso = IsometryMatrix3::<f64>::identity();"] # [doc = " let iso2 = iso.cast::<f32>();"] # [doc = " assert_eq!(iso2, IsometryMatrix3::<f32>::identity());"] # [doc = " ```"] pub fn cast < To : Scalar > (self) -> IsometryMatrix3 < To > where IsometryMatrix3 < To > : SupersetOf < Self > , { crate :: convert (self) } }
};
}
