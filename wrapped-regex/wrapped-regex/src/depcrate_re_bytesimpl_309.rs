// Generated macro for impl_309 (impl)
macro_rules! Depcrate_re_bytesimpl_309 {
() => {
// Module: crate::re_bytes
// Provides: {"impl_309"}
// Dependencies: {}
impl < 'r > Iterator for CaptureNames < 'r > { type Item = Option < & 'r str > ; fn next (& mut self) -> Option < Option < & 'r str > > { self . 0 . next () . as_ref () . map (| slot | slot . as_ref () . map (| name | name . as_ref ())) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
};
}
