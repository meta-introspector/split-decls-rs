// Generated macro for reserve_does_not_overallocate (function)
macro_rules! Depcrate_raw_vec_testsreserve_does_not_overallocate {
() => {
// Module: crate::raw_vec::tests
// Provides: {"reserve_does_not_overallocate"}
// Dependencies: {}
# [test] fn reserve_does_not_overallocate () { { let mut v : RawVec < u32 > = RawVec :: new () ; v . reserve (0 , 9) ; assert_eq ! (9 , v . capacity ()) ; } { let mut v : RawVec < u32 > = RawVec :: new () ; v . reserve (0 , 7) ; assert_eq ! (7 , v . capacity ()) ; v . reserve (7 , 90) ; assert_eq ! (97 , v . capacity ()) ; } { let mut v : RawVec < u32 > = RawVec :: new () ; v . reserve (0 , 12) ; assert_eq ! (12 , v . capacity ()) ; v . reserve (12 , 3) ; assert ! (v . capacity () >= 12 + 12 / 2) ; } }
};
}
