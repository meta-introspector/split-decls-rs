// Generated macro for VecShrinker (struct)
macro_rules! Depcrate_arbitraryVecShrinker {
() => {
// Module: crate::arbitrary
// Provides: {"VecShrinker"}
// Dependencies: {}
# [doc = "Iterator which returns successive attempts to shrink the vector `seed`"] struct VecShrinker < A > { seed : Vec < A > , # [doc = " How much which is removed when trying with smaller vectors"] size : usize , # [doc = " The end of the removed elements"] offset : usize , # [doc = " The shrinker for the element at `offset` once shrinking of individual"] # [doc = " elements are attempted"] element_shrinker : Box < dyn Iterator < Item = A > > , }
};
}
