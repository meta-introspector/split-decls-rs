// Generated macro for impl_564 (impl)
macro_rules! Depcrate_pat_utilimpl_564 {
() => {
// Module: crate::pat_util
// Provides: {"impl_564"}
// Dependencies: {}
impl < I > Iterator for EnumerateAndAdjust < I > where I : Iterator , { type Item = (usize , < I as Iterator > :: Item) ; fn next (& mut self) -> Option < (usize , < I as Iterator > :: Item) > { self . enumerate . next () . map (| (i , elem) | (if i < self . gap_pos { i } else { i + self . gap_len } , elem)) } fn size_hint (& self) -> (usize , Option < usize >) { self . enumerate . size_hint () } }
};
}
