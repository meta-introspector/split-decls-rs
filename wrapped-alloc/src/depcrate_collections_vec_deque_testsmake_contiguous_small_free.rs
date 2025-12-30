// Generated macro for make_contiguous_small_free (function)
macro_rules! Depcrate_collections_vec_deque_testsmake_contiguous_small_free {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"make_contiguous_small_free"}
// Dependencies: {}
# [test] fn make_contiguous_small_free () { let mut tester = VecDeque :: with_capacity (16) ; for i in b'A' .. b'I' { tester . push_back (i as char) ; } for i in b'I' .. b'N' { tester . push_front (i as char) ; } assert_eq ! (tester , ['M' , 'L' , 'K' , 'J' , 'I' , 'A' , 'B' , 'C' , 'D' , 'E' , 'F' , 'G' , 'H']) ; let expected_start = 0 ; tester . make_contiguous () ; assert_eq ! (tester . head , expected_start) ; assert_eq ! ((& ['M' , 'L' , 'K' , 'J' , 'I' , 'A' , 'B' , 'C' , 'D' , 'E' , 'F' , 'G' , 'H'] as & [_] , & [] as & [_]) , tester . as_slices ()) ; tester . clear () ; for i in b'I' .. b'N' { tester . push_back (i as char) ; } for i in b'A' .. b'I' { tester . push_front (i as char) ; } let expected_start = 3 ; tester . make_contiguous () ; assert_eq ! (tester . head , expected_start) ; assert_eq ! ((& ['H' , 'G' , 'F' , 'E' , 'D' , 'C' , 'B' , 'A' , 'I' , 'J' , 'K' , 'L' , 'M'] as & [_] , & [] as & [_]) , tester . as_slices ()) ; }
};
}
