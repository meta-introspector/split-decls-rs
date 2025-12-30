// Generated macro for test_alloc_uninitialized (function)
macro_rules! Depcrate_testtest_alloc_uninitialized {
() => {
// Module: crate::test
// Provides: {"test_alloc_uninitialized"}
// Dependencies: {}
# [test] fn test_alloc_uninitialized () { const LIMIT : usize = 15 ; let drop_counter = Cell :: new (0) ; unsafe { let arena : Arena < Node > = Arena :: with_capacity (4) ; for i in 0 .. LIMIT { let slice = arena . alloc_uninitialized (i) ; for (j , elem) in slice . iter_mut () . enumerate () { ptr :: write (elem . as_mut_ptr () , Node (None , j as u32 , DropTracker (& drop_counter)) ,) ; } assert_eq ! (drop_counter . get () , 0) ; } } assert_eq ! (drop_counter . get () , (0 .. LIMIT) . fold (0 , | a , e | a + e) as u32) ; }
};
}
