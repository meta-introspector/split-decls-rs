// Generated macro for impl_221 (impl)
macro_rules! Depcrate_stringimpl_221 {
() => {
// Module: crate::string
// Provides: {"impl_221"}
// Dependencies: {}
impl < 'r > Iterator for CaptureNames < 'r > { type Item = Option < & 'r str > ; # [inline] fn next (& mut self) -> Option < Option < & 'r str > > { self . 0 . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } # [inline] fn count (self) -> usize { self . 0 . count () } }
};
}
