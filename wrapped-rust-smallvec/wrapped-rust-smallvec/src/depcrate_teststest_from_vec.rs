// Generated macro for test_from_vec (function)
macro_rules! Depcrate_teststest_from_vec {
() => {
// Module: crate::tests
// Provides: {"test_from_vec"}
// Dependencies: {}
# [test] fn test_from_vec () { let vec = vec ! [] ; let small_vec : SmallVec < u8 , 3 > = SmallVec :: from_vec (vec) ; assert_eq ! (&* small_vec , & []) ; drop (small_vec) ; let vec = vec ! [] ; let small_vec : SmallVec < u8 , 1 > = SmallVec :: from_vec (vec) ; assert_eq ! (&* small_vec , & []) ; drop (small_vec) ; let vec = vec ! [1] ; let small_vec : SmallVec < u8 , 3 > = SmallVec :: from_vec (vec) ; assert_eq ! (&* small_vec , & [1]) ; drop (small_vec) ; let vec = vec ! [1 , 2 , 3] ; let small_vec : SmallVec < u8 , 3 > = SmallVec :: from_vec (vec) ; assert_eq ! (&* small_vec , & [1 , 2 , 3]) ; drop (small_vec) ; let vec = vec ! [1 , 2 , 3 , 4 , 5] ; let small_vec : SmallVec < u8 , 3 > = SmallVec :: from_vec (vec) ; assert_eq ! (&* small_vec , & [1 , 2 , 3 , 4 , 5]) ; drop (small_vec) ; let vec = vec ! [1 , 2 , 3 , 4 , 5] ; let small_vec : SmallVec < u8 , 1 > = SmallVec :: from_vec (vec) ; assert_eq ! (&* small_vec , & [1 , 2 , 3 , 4 , 5]) ; drop (small_vec) ; }
};
}
