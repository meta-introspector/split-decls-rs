// Generated macro for test_alloc_extend_with_drop_counter (function)
macro_rules! Depcrate_testtest_alloc_extend_with_drop_counter {
() => {
// Module: crate::test
// Provides: {"test_alloc_extend_with_drop_counter"}
// Dependencies: {}
# [test] fn test_alloc_extend_with_drop_counter () { let drop_counter = Cell :: new (0) ; { let arena = Arena :: with_capacity (2) ; let iter = (0 .. 100) . map (| j | Node (None , j as u32 , DropTracker (& drop_counter))) ; let older_ref = Some (& arena . alloc_extend (iter) [0]) ; assert_eq ! (drop_counter . get () , 0) ; let iter = (0 .. 100) . map (| j | Node (older_ref , j as u32 , DropTracker (& drop_counter))) ; arena . alloc_extend (iter) ; assert_eq ! (drop_counter . get () , 0) ; } assert_eq ! (drop_counter . get () , 200) ; }
};
}
