// Generated macro for split_at_unchecked (function)
macro_rules! Depcrate_slicesplit_at_unchecked {
() => {
// Module: crate::slice
// Provides: {"split_at_unchecked"}
// Dependencies: {}
# [doc = " Unchecked version of `xs.split_at(i)`."] unsafe fn split_at_unchecked < T > (xs : & [T] , i : usize) -> (& [T] , & [T]) { (get_unchecked (xs , .. i) , get_unchecked (xs , i ..)) }
};
}
