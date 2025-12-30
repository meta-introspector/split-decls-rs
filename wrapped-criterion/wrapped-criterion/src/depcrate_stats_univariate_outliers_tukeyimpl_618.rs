// Generated macro for impl_618 (impl)
macro_rules! Depcrate_stats_univariate_outliers_tukeyimpl_618 {
() => {
// Module: crate::stats::univariate::outliers::tukey
// Provides: {"impl_618"}
// Dependencies: {}
impl < 'a , A > IntoIterator for & LabeledSample < 'a , A > where A : Float , { type Item = (A , Label) ; type IntoIter = Iter < 'a , A > ; fn into_iter (self) -> Iter < 'a , A > { self . iter () } }
};
}
