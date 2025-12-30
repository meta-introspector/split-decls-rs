// Generated macro for factorial (function)
macro_rules! Depcrate_linalg_expfactorial {
() => {
// Module: crate::linalg::exp
// Provides: {"factorial"}
// Dependencies: {}
# [doc = " Compute `n!`"] # [inline (always)] fn factorial (n : usize) -> u128 { match FACTORIAL . get (n) { Some (f) => * f , None => panic ! ("{}! is greater than u128::MAX" , n) , } }
};
}
