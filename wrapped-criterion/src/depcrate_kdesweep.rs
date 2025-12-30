// Generated macro for sweep (function)
macro_rules! Depcrate_kdesweep {
() => {
// Module: crate::kde
// Provides: {"sweep"}
// Dependencies: {}
pub fn sweep (sample : & Sample < f64 > , npoints : usize , range : Option < (f64 , f64) > ,) -> (Box < [f64] > , Box < [f64] >) { let (xs , ys , _) = sweep_and_estimate (sample , npoints , range , sample [0]) ; (xs , ys) }
};
}
