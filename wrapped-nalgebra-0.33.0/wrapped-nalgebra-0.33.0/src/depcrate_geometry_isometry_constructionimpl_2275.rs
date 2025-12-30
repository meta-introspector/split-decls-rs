// Generated macro for impl_2275 (impl)
macro_rules! Depcrate_geometry_isometry_constructionimpl_2275 {
() => {
// Module: crate::geometry::isometry_construction
// Provides: {"impl_2275"}
// Dependencies: {}
# [doc = " # Construction from a 3D vector and/or an axis-angle"] impl < T : SimdRealField > Isometry3 < T > where T :: Element : SimdRealField , { basic_isometry_construction_impl ! (UnitQuaternion < T >) ; # [doc = " Cast the components of `self` to another type."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::Isometry3;"] # [doc = " let iso = Isometry3::<f64>::identity();"] # [doc = " let iso2 = iso.cast::<f32>();"] # [doc = " assert_eq!(iso2, Isometry3::<f32>::identity());"] # [doc = " ```"] pub fn cast < To : Scalar > (self) -> Isometry3 < To > where Isometry3 < To > : SupersetOf < Self > , { crate :: convert (self) } }
};
}
