// Generated macro for impl_278 (impl)
macro_rules! Depcrate_ansiimpl_278 {
() => {
// Module: crate::ansi
// Provides: {"impl_278"}
// Dependencies: {}
impl < 'a > Iterator for Matches < 'a > { type Item = Match < 'a > ; fn next (& mut self) -> Option < Self :: Item > { find_ansi_code_exclusive (& mut self . it) . map (| (start , end) | Match { text : self . s , start , end , }) } }
};
}
