// Generated macro for make_contiguous_head_to_end (function)
macro_rules! Depcrate_collections_vec_deque_testsmake_contiguous_head_to_end {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"make_contiguous_head_to_end"}
// Dependencies: {}
# [test] fn make_contiguous_head_to_end () { let mut tester = VecDeque :: with_capacity (16) ; for i in b'A' .. b'L' { tester . push_back (i as char) ; } for i in b'L' .. b'Q' { tester . push_front (i as char) ; } assert_eq ! (tester , ['P' , 'O' , 'N' , 'M' , 'L' , 'A' , 'B' , 'C' , 'D' , 'E' , 'F' , 'G' , 'H' , 'I' , 'J' , 'K']) ; let expected_start = 0 ; tester . make_contiguous () ; assert_eq ! (tester . head , expected_start) ; assert_eq ! ((& ['P' , 'O' , 'N' , 'M' , 'L' , 'A' , 'B' , 'C' , 'D' , 'E' , 'F' , 'G' , 'H' , 'I' , 'J' , 'K'] as & [_] , & [] as & [_]) , tester . as_slices ()) ; tester . clear () ; for i in b'L' .. b'Q' { tester . push_back (i as char) ; } for i in b'A' .. b'L' { tester . push_front (i as char) ; } let expected_start = 0 ; tester . make_contiguous () ; assert_eq ! (tester . head , expected_start) ; assert_eq ! ((& ['K' , 'J' , 'I' , 'H' , 'G' , 'F' , 'E' , 'D' , 'C' , 'B' , 'A' , 'L' , 'M' , 'N' , 'O' , 'P'] as & [_] , & [] as & [_]) , tester . as_slices ()) ; }
};
}
