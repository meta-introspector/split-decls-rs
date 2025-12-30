// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn iter_is_sorted () { let mut hf = HardForks :: default () ; hf . register (30) ; hf . register (20) ; hf . register (10) ; hf . register (20) ; assert_eq ! (hf . hard_forks , vec ! [(10 , 1) , (20 , 2) , (30 , 1)]) ; } # [test] fn multiple_hard_forks_since_parent () { let mut hf = HardForks :: default () ; hf . register (10) ; hf . register (20) ; assert_eq ! (hf . get_hash_data (9 , 0) , None) ; assert_eq ! (hf . get_hash_data (10 , 0) , Some ([1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,])) ; assert_eq ! (hf . get_hash_data (19 , 0) , Some ([1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,])) ; assert_eq ! (hf . get_hash_data (20 , 0) , Some ([2 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,])) ; assert_eq ! (hf . get_hash_data (20 , 10) , Some ([1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,])) ; assert_eq ! (hf . get_hash_data (20 , 11) , Some ([1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,])) ; assert_eq ! (hf . get_hash_data (21 , 11) , Some ([1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,])) ; assert_eq ! (hf . get_hash_data (21 , 20) , None) ; } }
};
}
