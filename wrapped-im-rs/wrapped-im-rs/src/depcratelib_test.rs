// Generated macro for lib_test (module)
macro_rules! Depcratelib_test {
() => {
// Module: crate
// Provides: {"lib_test"}
// Dependencies: {}
# [cfg (test)] mod lib_test { # [test] fn update_in () { let vector = vector ! [1 , 2 , 3 , 4 , 5] ; assert_eq ! (vector ! [1 , 2 , 23 , 4 , 5] , update_in ! (vector , 2 , 23)) ; let hashmap = hashmap ! [1 => 1 , 2 => 2 , 3 => 3] ; assert_eq ! (hashmap ! [1 => 1 , 2 => 23 , 3 => 3] , update_in ! (hashmap , 2 , 23)) ; let ordmap = ordmap ! [1 => 1 , 2 => 2 , 3 => 3] ; assert_eq ! (ordmap ! [1 => 1 , 2 => 23 , 3 => 3] , update_in ! (ordmap , 2 , 23)) ; let vecs = vector ! [vector ! [1 , 2 , 3] , vector ! [4 , 5 , 6] , vector ! [7 , 8 , 9]] ; let vecs_target = vector ! [vector ! [1 , 2 , 3] , vector ! [4 , 5 , 23] , vector ! [7 , 8 , 9]] ; assert_eq ! (vecs_target , update_in ! (vecs , 1 => 2 , 23)) ; } # [test] fn get_in () { let vector = vector ! [1 , 2 , 3 , 4 , 5] ; assert_eq ! (Some (& 3) , get_in ! (vector , 2)) ; let hashmap = hashmap ! [1 => 1 , 2 => 2 , 3 => 3] ; assert_eq ! (Some (& 2) , get_in ! (hashmap , & 2)) ; let ordmap = ordmap ! [1 => 1 , 2 => 2 , 3 => 3] ; assert_eq ! (Some (& 2) , get_in ! (ordmap , & 2)) ; let vecs = vector ! [vector ! [1 , 2 , 3] , vector ! [4 , 5 , 6] , vector ! [7 , 8 , 9]] ; assert_eq ! (Some (& 6) , get_in ! (vecs , 1 => 2)) ; } }
};
}
