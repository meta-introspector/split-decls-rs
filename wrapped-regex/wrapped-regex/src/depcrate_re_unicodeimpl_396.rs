// Generated macro for impl_396 (impl)
macro_rules! Depcrate_re_unicodeimpl_396 {
() => {
// Module: crate::re_unicode
// Provides: {"impl_396"}
// Dependencies: {}
impl < 'c > Iterator for SubCaptures < 'c > { type Item = Option < & 'c str > ; fn next (& mut self) -> Option < Option < & 'c str > > { if self . idx < self . caps . len () { self . idx += 1 ; Some (self . caps . at (self . idx - 1)) } else { None } } }
};
}
