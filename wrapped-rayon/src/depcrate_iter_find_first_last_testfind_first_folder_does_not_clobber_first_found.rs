// Generated macro for find_first_folder_does_not_clobber_first_found (function)
macro_rules! Depcrate_iter_find_first_last_testfind_first_folder_does_not_clobber_first_found {
() => {
// Module: crate::iter::find_first_last::test
// Provides: {"find_first_folder_does_not_clobber_first_found"}
// Dependencies: {}
# [test] fn find_first_folder_does_not_clobber_first_found () { let best_found = AtomicUsize :: new (usize :: MAX) ; let f = FindFolder { find_op : & (| & _ : & i32 | -> bool { true }) , boundary : 0 , match_position : MatchPosition :: Leftmost , best_found : & best_found , item : None , } ; let f = f . consume (0_i32) . consume (1_i32) . consume (2_i32) ; assert ! (f . full ()) ; assert_eq ! (f . complete () , Some (0_i32)) ; }
};
}
