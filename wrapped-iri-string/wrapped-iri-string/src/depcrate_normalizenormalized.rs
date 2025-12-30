// Generated macro for Normalized (struct)
macro_rules! Depcrate_normalizeNormalized {
() => {
// Module: crate::normalize
// Provides: {"Normalized"}
// Dependencies: {}
# [doc = " Normalized OR resolved IRI."] # [doc = ""] # [doc = " Resolved IRI can be represented by this type. In that case, the result might"] # [doc = " not be normalized. If you want the IRI resolution result to be normalized,"] # [doc = " use [`enable_normalization`][`Self::enable_normalization`] method."] # [doc = ""] # [doc = " [`Display`]: `core::fmt::Display`"] pub struct Normalized < 'a , T : ? Sized > { # [doc = " Spec-agnostic normalization input."] input : NormalizationInput < 'a > , # [doc = " Expected result type."] _ty_str : PhantomData < fn () -> T > , }
};
}
