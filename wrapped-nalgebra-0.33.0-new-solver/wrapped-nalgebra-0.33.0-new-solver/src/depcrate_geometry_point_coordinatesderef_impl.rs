// Generated macro for deref_impl (macro)
macro_rules! Depcrate_geometry_point_coordinatesderef_impl {
() => {
// Module: crate::geometry::point_coordinates
// Provides: {"deref_impl"}
// Dependencies: {}
macro_rules ! deref_impl (($ D : ty , $ Target : ident $ (, $ comps : ident) *) => { impl < T : Scalar > Deref for OPoint < T , $ D > { type Target = $ Target < T >; # [inline] fn deref (& self) -> & Self :: Target { &* self . coords } } impl < T : Scalar > DerefMut for OPoint < T , $ D > { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { & mut * self . coords } } }) ;
};
}
