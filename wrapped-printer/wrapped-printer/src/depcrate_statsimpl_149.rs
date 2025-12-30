// Generated macro for impl_149 (impl)
macro_rules! Depcrate_statsimpl_149 {
() => {
// Module: crate::stats
// Provides: {"impl_149"}
// Dependencies: {}
impl < 'a > Add < & 'a Stats > for Stats { type Output = Stats ; fn add (self , rhs : & 'a Stats) -> Stats { Stats { elapsed : NiceDuration (self . elapsed . 0 + rhs . elapsed . 0) , searches : self . searches + rhs . searches , searches_with_match : self . searches_with_match + rhs . searches_with_match , bytes_searched : self . bytes_searched + rhs . bytes_searched , bytes_printed : self . bytes_printed + rhs . bytes_printed , matched_lines : self . matched_lines + rhs . matched_lines , matches : self . matches + rhs . matches , } } }
};
}
