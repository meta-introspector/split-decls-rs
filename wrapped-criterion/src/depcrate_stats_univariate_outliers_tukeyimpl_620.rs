// Generated macro for impl_620 (impl)
macro_rules! Depcrate_stats_univariate_outliers_tukeyimpl_620 {
() => {
// Module: crate::stats::univariate::outliers::tukey
// Provides: {"impl_620"}
// Dependencies: {}
impl < 'a , A > Iterator for Iter < 'a , A > where A : Float , { type Item = (A , Label) ; # [allow (clippy :: similar_names)] fn next (& mut self) -> Option < (A , Label) > { self . iter . next () . map (| & x | { let (lost , lomt , himt , hist) = self . fences ; let label = if x < lost { LowSevere } else if x > hist { HighSevere } else if x < lomt { LowMild } else if x > himt { HighMild } else { NotAnOutlier } ; (x , label) }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
