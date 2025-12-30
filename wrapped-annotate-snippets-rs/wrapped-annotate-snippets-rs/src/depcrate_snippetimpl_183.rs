// Generated macro for impl_183 (impl)
macro_rules! Depcrate_snippetimpl_183 {
() => {
// Module: crate::snippet
// Provides: {"impl_183"}
// Dependencies: {}
impl < 'a > Patch < 'a > { # [doc = " Splice `replacement` into the [`Snippet`] at the specified byte span"] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " Text passed to this function is considered \"untrusted input\", as such"] # [doc = " all text is passed through a normalization function. Pre-styled text is"] # [doc = " not allowed to be passed to this function."] # [doc = ""] # [doc = " </div>"] pub fn new (span : Range < usize > , replacement : impl Into < Cow < 'a , str > >) -> Self { Self { span , replacement : replacement . into () , } } # [doc = " Try to turn a replacement into an addition when the span that is being"] # [doc = " overwritten matches either the prefix or suffix of the replacement."] pub (crate) fn trim_trivial_replacements (self , source : & str) -> TrimmedPatch < 'a > { let mut trimmed = TrimmedPatch { original_span : self . span . clone () , span : self . span , replacement : self . replacement , } ; if trimmed . replacement . is_empty () { return trimmed ; } let Some (snippet) = source . get (trimmed . original_span . clone ()) else { return trimmed ; } ; if let Some ((prefix , substr , suffix)) = as_substr (snippet , & trimmed . replacement) { trimmed . span = trimmed . original_span . start + prefix .. trimmed . original_span . end . saturating_sub (suffix) ; trimmed . replacement = Cow :: Owned (substr . to_owned ()) ; } trimmed } }
};
}
