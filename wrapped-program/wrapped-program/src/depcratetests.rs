// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: unchecked_div_by_const ; # [test] fn test_unchecked_div_by_const () { const D : u64 = 2 ; const N : u64 = 10 ; let n = 10 ; assert_eq ! (unchecked_div_by_const ! (10 , 2) , 5) ; assert_eq ! (unchecked_div_by_const ! (N , 2) , 5) ; assert_eq ! (unchecked_div_by_const ! (n , 2) , 5) ; assert_eq ! (unchecked_div_by_const ! (10 , D) , 5) ; assert_eq ! (unchecked_div_by_const ! (N , D) , 5) ; assert_eq ! (unchecked_div_by_const ! (n , D) , 5) ; } }
};
}
