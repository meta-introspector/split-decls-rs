// Generated macro for collect_collects (function)
macro_rules! Depcrate_allcollect_collects {
() => {
// Module: crate::all
// Provides: {"collect_collects"}
// Dependencies: {}
# [test] fn collect_collects () { assert_done (| | collect (vec ! [f_ok (1) , f_ok (2)]) , Ok (vec ! [1 , 2])) ; assert_done (| | collect (vec ! [f_ok (1)]) , Ok (vec ! [1])) ; assert_done (| | collect (Vec :: < Result < i32 , u32 > > :: new ()) , Ok (vec ! [])) ; }
};
}
