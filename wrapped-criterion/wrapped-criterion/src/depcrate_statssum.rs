// Generated macro for sum (function)
macro_rules! Depcrate_statssum {
() => {
// Module: crate::stats
// Provides: {"sum"}
// Dependencies: {}
fn sum < A > (xs : & [A]) -> A where A : Float , { use std :: ops :: Add ; xs . iter () . cloned () . fold (A :: cast (0) , Add :: add) }
};
}
