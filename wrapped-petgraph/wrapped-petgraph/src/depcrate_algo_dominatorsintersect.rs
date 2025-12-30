// Generated macro for intersect (function)
macro_rules! Depcrate_algo_dominatorsintersect {
() => {
// Module: crate::algo::dominators
// Provides: {"intersect"}
// Dependencies: {}
fn intersect (dominators : & [usize] , mut finger1 : usize , mut finger2 : usize) -> usize { loop { match finger1 . cmp (& finger2) { Ordering :: Less => finger1 = dominators [finger1] , Ordering :: Greater => finger2 = dominators [finger2] , Ordering :: Equal => return finger1 , } } }
};
}
