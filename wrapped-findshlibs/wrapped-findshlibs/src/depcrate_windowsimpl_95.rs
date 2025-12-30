// Generated macro for impl_95 (impl)
macro_rules! Depcrate_windowsimpl_95 {
() => {
// Module: crate::windows
// Provides: {"impl_95"}
// Dependencies: {}
impl < 'a > Iterator for SegmentIter < 'a > { type Item = Segment < 'a > ; fn next (& mut self) -> Option < Self :: Item > { self . sections . next () . map (| section | Segment { section }) } }
};
}
