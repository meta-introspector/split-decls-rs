// Generated macro for logsumexp (function)
macro_rules! Depcrate_safelogsumexp {
() => {
// Module: crate::safe
// Provides: {"logsumexp"}
// Dependencies: {}
# [inline] fn logsumexp (vect : & [f64]) -> f64 { let mut sum = 0.0 ; for & val in vect { sum += val . exp () ; } sum += 2.0 ; sum . ln () }
};
}
