// Generated macro for impl_228 (impl)
macro_rules! Depcrate_matcher_support_summarize_diffimpl_228 {
() => {
// Module: crate::matcher_support::summarize_diff
// Provides: {"impl_228"}
// Dependencies: {}
impl Display for BufferedSummary < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { if ! matches ! (self . buffer , Buffer :: Empty) { panic ! ("Buffer is not empty. This is a bug in gtest_rust.") } if ! self . summary . last_ansi_style . is_empty () { panic ! ("ANSI style has not been reset. This is a bug in gtest_rust.") } self . summary . summary . fmt (f) } }
};
}
