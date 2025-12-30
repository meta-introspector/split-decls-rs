// Generated macro for NormalizedInner (struct)
macro_rules! Depcrate_normalizeNormalizedInner {
() => {
// Module: crate::normalize
// Provides: {"NormalizedInner"}
// Dependencies: {}
# [doc = " Writable as a normalized IRI."] # [doc = ""] # [doc = " Note that this implicitly apply serialization rule defined by WHATWG URL"] # [doc = " Standard (to handle normalization impossible by RFC 3986) because `Display`"] # [doc = " should not fail by reasons other than backend I/O failure. If you make the"] # [doc = " normalization fail in such cases, check if the path starts with `/./`."] # [doc = " When the normalization succeeds by RFC 3986 algorithm, the path never starts"] # [doc = " with `/./`."] struct NormalizedInner < 'a , S > { # [doc = " Spec-agnostic normalization input."] input : NormalizationInput < 'a > , # [doc = " Spec."] _spec : PhantomData < fn () -> S > , }
};
}
