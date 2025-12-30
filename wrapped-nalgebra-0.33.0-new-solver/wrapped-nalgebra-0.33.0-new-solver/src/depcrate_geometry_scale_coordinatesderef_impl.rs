// Generated macro for deref_impl (macro)
macro_rules! Depcrate_geometry_scale_coordinatesderef_impl {
() => {
// Module: crate::geometry::scale_coordinates
// Provides: {"deref_impl"}
// Dependencies: {}
macro_rules ! deref_impl (($ D : expr , $ Target : ident $ (, $ comps : ident) *) => { impl < T : Scalar > Deref for Scale < T , $ D > { type Target = $ Target < T >; # [inline] fn deref (& self) -> & Self :: Target { self . vector . deref () } } impl < T : Scalar > DerefMut for Scale < T , $ D > { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { self . vector . deref_mut () } } }) ;
};
}
