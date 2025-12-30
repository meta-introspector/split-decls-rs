// Generated macro for zst (function)
macro_rules! Depcrate_raw_vec_testszst {
() => {
// Module: crate::raw_vec::tests
// Provides: {"zst"}
// Dependencies: {}
# [test] fn zst () { let cap_err = Err (crate :: collections :: TryReserveErrorKind :: CapacityOverflow . into ()) ; assert_eq ! (size_of ::< ZST > () , 0) ; let v : RawVec < ZST > = RawVec :: new () ; zst_sanity (& v) ; let v : RawVec < ZST > = RawVec :: with_capacity_in (100 , Global) ; zst_sanity (& v) ; let v : RawVec < ZST > = RawVec :: with_capacity_in (100 , Global) ; zst_sanity (& v) ; let mut v : RawVec < ZST > = RawVec :: with_capacity_in (usize :: MAX , Global) ; zst_sanity (& v) ; v . reserve (100 , usize :: MAX - 100) ; zst_sanity (& v) ; v . reserve_exact (100 , usize :: MAX - 100) ; zst_sanity (& v) ; assert_eq ! (v . try_reserve (100 , usize :: MAX - 100) , Ok (())) ; assert_eq ! (v . try_reserve (101 , usize :: MAX - 100) , cap_err) ; zst_sanity (& v) ; assert_eq ! (v . try_reserve_exact (100 , usize :: MAX - 100) , Ok (())) ; assert_eq ! (v . try_reserve_exact (101 , usize :: MAX - 100) , cap_err) ; zst_sanity (& v) ; assert_eq ! (v . inner . grow_amortized (100 , usize :: MAX - 100 , ZST :: LAYOUT) , cap_err) ; assert_eq ! (v . inner . grow_amortized (101 , usize :: MAX - 100 , ZST :: LAYOUT) , cap_err) ; zst_sanity (& v) ; assert_eq ! (v . inner . grow_exact (100 , usize :: MAX - 100 , ZST :: LAYOUT) , cap_err) ; assert_eq ! (v . inner . grow_exact (101 , usize :: MAX - 100 , ZST :: LAYOUT) , cap_err) ; zst_sanity (& v) ; }
};
}
