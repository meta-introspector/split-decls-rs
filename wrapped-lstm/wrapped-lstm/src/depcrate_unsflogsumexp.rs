// Generated macro for logsumexp (function)
macro_rules! Depcrate_unsflogsumexp {
() => {
// Module: crate::unsf
// Provides: {"logsumexp"}
// Dependencies: {}
unsafe fn logsumexp (vect : * const f64 , sz : usize) -> f64 { let mut sum : f64 = 0.0 ; for i in 0 .. sz { sum += (* vect . add (i)) . exp () ; } sum += 2.0 ; sum . ln () }
};
}
