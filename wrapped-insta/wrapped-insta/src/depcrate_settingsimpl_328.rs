// Generated macro for impl_328 (impl)
macro_rules! Depcrate_settingsimpl_328 {
() => {
// Module: crate::settings
// Provides: {"impl_328"}
// Dependencies: {}
# [cfg (feature = "redactions")] impl Redactions { # [doc = " Applies all redactions to the given content."] pub (crate) fn apply_to_content (& self , mut content : Content) -> Content { for (selector , redaction) in self . 0 . iter () { content = selector . redact (content , redaction) ; } content } }
};
}
