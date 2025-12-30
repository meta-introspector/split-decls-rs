// Generated macro for ensure_into_vec_maintains_order_of_allocation (function)
macro_rules! Depcrate_testensure_into_vec_maintains_order_of_allocation {
() => {
// Module: crate::test
// Provides: {"ensure_into_vec_maintains_order_of_allocation"}
// Dependencies: {}
# [test] fn ensure_into_vec_maintains_order_of_allocation () { let arena = Arena :: with_capacity (1) ; for & s in & ["t" , "e" , "s" , "t"] { arena . alloc (String :: from (s)) ; } let vec = arena . into_vec () ; assert_eq ! (vec , vec ! ["t" , "e" , "s" , "t"]) ; }
};
}
