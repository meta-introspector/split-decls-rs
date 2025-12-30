// Generated macro for impl_339 (impl)
macro_rules! Depcrate_subtags_languageimpl_339 {
() => {
// Module: crate::subtags::language
// Provides: {"impl_339"}
// Dependencies: {}
impl Language { # [doc = " The unknown language \"und\"."] pub const UNKNOWN : Self = language ! ("und") ; # [doc = " Whether this [`Language`] equals [`Language::UNKNOWN`]."] # [inline] pub const fn is_unknown (self) -> bool { matches ! (self , Self :: UNKNOWN) } }
};
}
