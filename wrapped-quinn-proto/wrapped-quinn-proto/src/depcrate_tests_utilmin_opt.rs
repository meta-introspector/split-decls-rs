// Generated macro for min_opt (function)
macro_rules! Depcrate_tests_utilmin_opt {
() => {
// Module: crate::tests::util
// Provides: {"min_opt"}
// Dependencies: {}
pub (super) fn min_opt < T : Ord > (x : Option < T > , y : Option < T >) -> Option < T > { match (x , y) { (Some (x) , Some (y)) => Some (cmp :: min (x , y)) , (Some (x) , _) => Some (x) , (_ , Some (y)) => Some (y) , _ => None , } }
};
}
