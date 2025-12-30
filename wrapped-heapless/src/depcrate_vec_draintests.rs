// Generated macro for tests (module)
macro_rules! Depcrate_vec_draintests {
() => {
// Module: crate::vec::drain
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: super :: Vec ; # [test] fn drain_front () { let mut vec = Vec :: < _ , 8 > :: from_array ([1 , 2 , 3 , 4]) ; let mut it = vec . drain (.. 1) ; assert_eq ! (it . next () , Some (1)) ; drop (it) ; assert_eq ! (vec , & [2 , 3 , 4]) ; } # [test] fn drain_middle () { let mut vec = Vec :: < _ , 8 > :: from_array ([1 , 2 , 3 , 4]) ; let mut it = vec . drain (1 .. 3) ; assert_eq ! (it . next () , Some (2)) ; assert_eq ! (it . next () , Some (3)) ; drop (it) ; assert_eq ! (vec , & [1 , 4]) ; } # [test] fn drain_end () { let mut vec = Vec :: < _ , 8 > :: from_array ([1 , 2 , 3 , 4]) ; let mut it = vec . drain (3 ..) ; assert_eq ! (it . next () , Some (4)) ; drop (it) ; assert_eq ! (vec , & [1 , 2 , 3]) ; } # [test] fn drain_drop_rest () { droppable ! () ; let mut vec = Vec :: < _ , 8 > :: from_array ([Droppable :: new () , Droppable :: new () , Droppable :: new () , Droppable :: new () ,]) ; assert_eq ! (Droppable :: count () , 4) ; let mut iter = vec . drain (2 ..) ; assert_eq ! (iter . next () . unwrap () . 0 , 3) ; drop (iter) ; assert_eq ! (Droppable :: count () , 2) ; assert_eq ! (vec . len () , 2) ; assert_eq ! (vec . remove (0) . 0 , 1) ; assert_eq ! (Droppable :: count () , 1) ; drop (vec) ; assert_eq ! (Droppable :: count () , 0) ; } }
};
}
