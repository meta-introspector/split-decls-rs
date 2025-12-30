// Generated macro for shrink_after_from_empty_vec (function)
macro_rules! Depcrate_testsshrink_after_from_empty_vec {
() => {
// Module: crate::tests
// Provides: {"shrink_after_from_empty_vec"}
// Dependencies: {}
# [test] fn shrink_after_from_empty_vec () { let mut v = SmallVec :: < u8 , 2 > :: from_vec (vec ! []) ; v . shrink_to_fit () ; assert ! (! v . spilled ()) }
};
}
