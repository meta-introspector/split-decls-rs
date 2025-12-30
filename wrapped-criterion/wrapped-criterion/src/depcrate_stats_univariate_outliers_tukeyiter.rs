// Generated macro for Iter (struct)
macro_rules! Depcrate_stats_univariate_outliers_tukeyIter {
() => {
// Module: crate::stats::univariate::outliers::tukey
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " Iterator over the labeled data"] pub struct Iter < 'a , A > where A : Float , { fences : (A , A , A , A) , iter : slice :: Iter < 'a , A > , }
};
}
