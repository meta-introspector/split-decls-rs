// Generated macro for shrink_to_fit_unspill (function)
macro_rules! Depcrate_testsshrink_to_fit_unspill {
() => {
// Module: crate::tests
// Provides: {"shrink_to_fit_unspill"}
// Dependencies: {}
# [test] fn shrink_to_fit_unspill () { let mut vec = SmallVec :: < u8 , 2 > :: from_iter (0 .. 3) ; vec . pop () ; assert ! (vec . spilled ()) ; vec . shrink_to_fit () ; assert ! (! vec . spilled () , "shrink_to_fit will un-spill if possible") ; }
};
}
