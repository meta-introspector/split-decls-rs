// Generated macro for Kde (struct)
macro_rules! Depcrate_stats_univariate_kdeKde {
() => {
// Module: crate::stats::univariate::kde
// Provides: {"Kde"}
// Dependencies: {}
# [doc = " Univariate kernel density estimator"] pub struct Kde < 'a , A , K > where A : Float , K : Kernel < A > , { bandwidth : A , kernel : K , sample : & 'a Sample < A > , }
};
}
