// Generated macro for deref_impl (macro)
macro_rules! Depcrate_geometry_translation_coordinatesderef_impl {
() => {
// Module: crate::geometry::translation_coordinates
// Provides: {"deref_impl"}
// Dependencies: {}
macro_rules ! deref_impl (($ D : expr , $ Target : ident $ (, $ comps : ident) *) => { impl < T : Scalar > Deref for Translation < T , $ D > { type Target = $ Target < T >; # [inline] fn deref (& self) -> & Self :: Target { unsafe { &* (self as * const Translation < T , $ D > as * const Self :: Target) } } } impl < T : Scalar > DerefMut for Translation < T , $ D > { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { & mut * (self as * mut Translation < T , $ D > as * mut Self :: Target) } } } }) ;
};
}
