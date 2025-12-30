// Generated macro for mul_size_hints (function)
macro_rules! Depcrate_size_hintmul_size_hints {
() => {
// Module: crate::size_hint
// Provides: {"mul_size_hints"}
// Dependencies: {}
# [doc = " Test multiplication of size hints."] # [test] fn mul_size_hints () { assert_eq ! (mul ((3 , Some (4)) , (3 , Some (4))) , (9 , Some (16))) ; assert_eq ! (mul ((3 , Some (4)) , (usize :: MAX , None)) , (usize :: MAX , None)) ; assert_eq ! (mul ((3 , None) , (0 , Some (0))) , (0 , Some (0))) ; }
};
}
