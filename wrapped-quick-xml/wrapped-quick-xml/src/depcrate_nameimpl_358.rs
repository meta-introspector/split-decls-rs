// Generated macro for impl_358 (impl)
macro_rules! Depcrate_nameimpl_358 {
() => {
// Module: crate::name
// Provides: {"impl_358"}
// Dependencies: {}
impl < 'a > Namespace < 'a > { # [doc = " Converts this namespace to an internal slice representation."] # [doc = ""] # [doc = " This is [non-normalized] attribute value, i.e. any entity references is"] # [doc = " not expanded and space characters are not removed. This means, that"] # [doc = " different byte slices, returned from this method, can represent the same"] # [doc = " namespace and would be treated by parser as identical."] # [doc = ""] # [doc = " For example, if the entity **eacute** has been defined to be **é**,"] # [doc = " the empty tags below all contain namespace declarations binding the"] # [doc = " prefix `p` to the same [IRI reference], `http://example.org/rosé`."] # [doc = ""] # [doc = " ```xml"] # [doc = " <p:foo xmlns:p=\"http://example.org/rosé\" />"] # [doc = " <p:foo xmlns:p=\"http://example.org/ros&#xe9;\" />"] # [doc = " <p:foo xmlns:p=\"http://example.org/ros&#xE9;\" />"] # [doc = " <p:foo xmlns:p=\"http://example.org/ros&#233;\" />"] # [doc = " <p:foo xmlns:p=\"http://example.org/ros&eacute;\" />"] # [doc = " ```"] # [doc = ""] # [doc = " This is because XML entity references are expanded during attribute value"] # [doc = " normalization."] # [doc = ""] # [doc = " [non-normalized]: https://www.w3.org/TR/xml11/#AVNormalize"] # [doc = " [IRI reference]: https://datatracker.ietf.org/doc/html/rfc3987"] # [inline (always)] pub const fn into_inner (self) -> & 'a [u8] { self . 0 } }
};
}
