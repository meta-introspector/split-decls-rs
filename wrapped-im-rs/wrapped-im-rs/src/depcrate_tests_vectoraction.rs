// Generated macro for Action (enum)
macro_rules! Depcrate_tests_vectorAction {
() => {
// Module: crate::tests::vector
// Provides: {"Action"}
// Dependencies: {}
# [derive (Arbitrary , Debug)] enum Action < A > { PushFront (A) , PushBack (A) , PopFront , PopBack , Insert (usize , A) , Remove (usize) , JoinLeft (Vec < A >) , JoinRight (Vec < A >) , SplitLeft (usize) , SplitRight (usize) , }
};
}
