// Generated macro for saturating_drain_iter (function)
macro_rules! Depcrate_blinksaturating_drain_iter {
() => {
// Module: crate::blink
// Provides: {"saturating_drain_iter"}
// Dependencies: {}
# [inline] fn saturating_drain_iter < T > (mut iter : impl Iterator < Item = T >) -> usize { let mut drained = 0 ; loop { let (lower , _) = iter . size_hint () ; if lower == 0 { match iter . next () { None => return drained , Some (_) => drained += 1 , } continue ; } let lower = lower . min (usize :: MAX - drained) ; match iter . nth (lower - 1) { None => { return drained ; } Some (_) => { drained += lower ; } } if drained == usize :: MAX { return usize :: MAX ; } } }
};
}
