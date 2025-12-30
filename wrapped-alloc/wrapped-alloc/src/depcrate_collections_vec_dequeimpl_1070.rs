// Generated macro for impl_1070 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_1070 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_1070"}
// Dependencies: {}
# [stable (feature = "std_collections_from_array" , since = "1.56.0")] impl < T , const N : usize > From < [T ; N] > for VecDeque < T > { # [doc = " Converts a `[T; N]` into a `VecDeque<T>`."] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::VecDeque;"] # [doc = ""] # [doc = " let deq1 = VecDeque::from([1, 2, 3, 4]);"] # [doc = " let deq2: VecDeque<_> = [1, 2, 3, 4].into();"] # [doc = " assert_eq!(deq1, deq2);"] # [doc = " ```"] # [track_caller] fn from (arr : [T ; N]) -> Self { let mut deq = VecDeque :: with_capacity (N) ; let arr = ManuallyDrop :: new (arr) ; if ! < T > :: IS_ZST { unsafe { ptr :: copy_nonoverlapping (arr . as_ptr () , deq . ptr () , N) ; } } deq . head = 0 ; deq . len = N ; deq } }
};
}
