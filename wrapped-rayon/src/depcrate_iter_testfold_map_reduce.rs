// Generated macro for fold_map_reduce (function)
macro_rules! Depcrate_iter_testfold_map_reduce {
() => {
// Module: crate::iter::test
// Provides: {"fold_map_reduce"}
// Dependencies: {}
# [test] fn fold_map_reduce () { let r1 = (0_i32 .. 32) . into_par_iter () . with_max_len (1) . fold (Vec :: new , | mut v , e | { v . push (e) ; v }) . map (| v | vec ! [v]) . reduce_with (| mut v_a , v_b | { v_a . extend (v_b) ; v_a }) ; assert_eq ! (r1 , Some (vec ! [vec ! [0] , vec ! [1] , vec ! [2] , vec ! [3] , vec ! [4] , vec ! [5] , vec ! [6] , vec ! [7] , vec ! [8] , vec ! [9] , vec ! [10] , vec ! [11] , vec ! [12] , vec ! [13] , vec ! [14] , vec ! [15] , vec ! [16] , vec ! [17] , vec ! [18] , vec ! [19] , vec ! [20] , vec ! [21] , vec ! [22] , vec ! [23] , vec ! [24] , vec ! [25] , vec ! [26] , vec ! [27] , vec ! [28] , vec ! [29] , vec ! [30] , vec ! [31]])) ; }
};
}
