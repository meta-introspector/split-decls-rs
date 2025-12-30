// Generated macro for impl_35 (impl)
macro_rules! Depcrate_locale_familyimpl_35 {
() => {
// Module: crate::locale_family
// Provides: {"impl_35"}
// Dependencies: {}
impl DataLocaleFamilyAnnotations { # [inline] pub (crate) const fn with_descendants () -> Self { Self { include_ancestors : true , include_descendants : true , } } # [inline] pub (crate) const fn without_descendants () -> Self { Self { include_ancestors : true , include_descendants : false , } } # [inline] pub (crate) const fn without_ancestors () -> Self { Self { include_ancestors : false , include_descendants : true , } } # [inline] pub (crate) const fn single () -> Self { Self { include_ancestors : false , include_descendants : false , } } }
};
}
