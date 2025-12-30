// Generated macro for build_argvec (function)
macro_rules! Depcrate_ifacedescbuild_argvec {
() => {
// Module: crate::ifacedesc
// Provides: {"build_argvec"}
// Dependencies: {}
fn build_argvec < A : arg :: ArgAll > (a : A :: strs) -> Arguments { let mut v = vec ! () ; A :: strs_sig (a , | name , sig | { v . push (Argument { name : name . into () , sig , annotations : Default :: default () }) }) ; Arguments (v) }
};
}
