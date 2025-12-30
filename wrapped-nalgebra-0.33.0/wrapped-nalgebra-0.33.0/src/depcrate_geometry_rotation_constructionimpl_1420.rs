// Generated macro for impl_1420 (impl)
macro_rules! Depcrate_geometry_rotation_constructionimpl_1420 {
() => {
// Module: crate::geometry::rotation_construction
// Provides: {"impl_1420"}
// Dependencies: {}
impl < T : Scalar , const D : usize > Rotation < T , D > { # [doc = " Cast the components of `self` to another type."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::Rotation2;"] # [doc = " let rot = Rotation2::<f64>::identity();"] # [doc = " let rot2 = rot.cast::<f32>();"] # [doc = " assert_eq!(rot2, Rotation2::<f32>::identity());"] # [doc = " ```"] pub fn cast < To : Scalar > (self) -> Rotation < To , D > where Rotation < To , D > : SupersetOf < Self > , { crate :: convert (self) } }
};
}
