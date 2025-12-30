// Generated macro for impl_1504 (impl)
macro_rules! Depcrate_stringimpl_1504 {
() => {
// Module: crate::string
// Provides: {"impl_1504"}
// Dependencies: {}
# [unstable (feature = "string_into_chars" , issue = "133125")] impl Iterator for IntoChars { type Item = char ; # [inline] fn next (& mut self) -> Option < char > { let mut iter = self . iter () ; match iter . next () { None => None , Some ((_ , ch)) => { let offset = iter . offset () ; let _ = self . bytes . advance_by (offset) ; Some (ch) } } } # [inline] fn count (self) -> usize { self . iter () . count () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter () . size_hint () } # [inline] fn last (mut self) -> Option < char > { self . next_back () } }
};
}
