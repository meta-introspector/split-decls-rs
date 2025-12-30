// Generated macro for impl_151 (impl)
macro_rules! Depcrate_statsimpl_151 {
() => {
// Module: crate::stats
// Provides: {"impl_151"}
// Dependencies: {}
impl < 'a > AddAssign < & 'a Stats > for Stats { fn add_assign (& mut self , rhs : & 'a Stats) { self . elapsed . 0 += rhs . elapsed . 0 ; self . searches += rhs . searches ; self . searches_with_match += rhs . searches_with_match ; self . bytes_searched += rhs . bytes_searched ; self . bytes_printed += rhs . bytes_printed ; self . matched_lines += rhs . matched_lines ; self . matches += rhs . matches ; } }
};
}
